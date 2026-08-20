use alloy::providers::DynProvider;
use alloy::providers::Provider;
use alloy::rpc::types::TransactionTrait;
use anoma_pa_evm_bindings::generated::protocol_adapter::{
    IProtocolAdapter, ProtocolAdapter as PaContract,
};
use anyhow::Context;

pub(super) const ERROR_STRING_SELECTOR: [u8; 4] = [0x08, 0xc3, 0x79, 0xa0];
pub(super) const PANIC_SELECTOR: [u8; 4] = [0x4e, 0x48, 0x7b, 0x71];

pub(in crate::envs) async fn execute_on_pa(
    pa: &PaContract::ProtocolAdapterInstance<DynProvider>,
    tx: IProtocolAdapter::Transaction,
) -> anyhow::Result<alloy::rpc::types::TransactionReceipt> {
    preflight_check(pa, tx.clone()).await?;

    let receipt = pa
        .execute(tx)
        .send()
        .await
        .context("failed to submit protocol adapter execution transaction")?
        .get_receipt()
        .await
        .context("failed to fetch protocol adapter execution receipt")?;

    if !receipt.status() {
        let gas_diagnostic = gas_diagnostic(pa, &receipt).await;
        anyhow::bail!(
            "protocol adapter execution failed: tx_hash={:?}, block_number={:?}, \
             gas_used={}, effective_gas_price={:?}, logs={}, \
             gas_diagnostic={}",
            receipt.transaction_hash,
            receipt.block_number,
            receipt.gas_used,
            receipt.effective_gas_price,
            receipt.logs().len(),
            gas_diagnostic,
        );
    }

    Ok(receipt)
}

async fn preflight_check(
    pa: &PaContract::ProtocolAdapterInstance<DynProvider>,
    tx: IProtocolAdapter::Transaction,
) -> anyhow::Result<()> {
    match pa.execute(tx).call().await {
        Ok(_) => Ok(()),
        Err(err) => anyhow::bail!(
            "protocol adapter preflight failed: {}",
            decode_revert_detail(&err.to_string())
        ),
    }
}

async fn gas_diagnostic(
    pa: &PaContract::ProtocolAdapterInstance<DynProvider>,
    receipt: &alloy::rpc::types::TransactionReceipt,
) -> String {
    let tx_lookup = pa
        .provider()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await
        .context("failed to fetch protocol adapter transaction by hash for diagnostics");

    match tx_lookup {
        Ok(Some(chain_tx)) => {
            let gas_limit = chain_tx.gas_limit();
            if receipt.gas_used == gas_limit {
                format!("gas_limit={gas_limit}, possible_out_of_gas=true")
            } else {
                format!("gas_limit={gas_limit}, possible_out_of_gas=false")
            }
        }
        Ok(None) => "transaction not found by hash".to_string(),
        Err(err) => format!("lookup failed ({err:#})"),
    }
}

pub(super) fn decode_revert_detail(err: &str) -> String {
    let Some(raw) = find_hex_payload(err) else {
        return err.to_string();
    };

    let bytes = match hex::decode(&raw[2..]) {
        Ok(bytes) => bytes,
        Err(_) => return err.to_string(),
    };

    if bytes.len() < 4 {
        return format!("{err}; revert_data={raw}");
    }

    let selector = [bytes[0], bytes[1], bytes[2], bytes[3]];
    if selector == ERROR_STRING_SELECTOR
        && let Some(reason) = decode_error_string_payload(&bytes[4..])
    {
        return format!("execution reverted: {reason}");
    }

    if selector == PANIC_SELECTOR
        && bytes.len() >= 36
        && let Some(code) = decode_word_usize(&bytes[4..36])
    {
        return format!("execution panicked with code 0x{code:x}");
    }

    format!("{err}; selector=0x{}; revert_data={raw}", &raw[2..10])
}

pub(super) fn find_hex_payload(input: &str) -> Option<&str> {
    input
        .split(|c: char| c.is_whitespace() || c == ',' || c == ')' || c == '(')
        .find(|token| {
            token.starts_with("0x")
                && token.len() > 10
                && token.len() % 2 == 0
                && hex::decode(&token[2..]).is_ok()
        })
}

fn decode_error_string_payload(payload: &[u8]) -> Option<String> {
    if payload.len() < 64 {
        return None;
    }

    let offset = decode_word_usize(&payload[0..32])?;
    if offset + 32 > payload.len() {
        return None;
    }

    let len = decode_word_usize(&payload[offset..offset + 32])?;
    let start = offset + 32;
    let end = start.checked_add(len)?;
    if end > payload.len() {
        return None;
    }

    std::str::from_utf8(&payload[start..end])
        .ok()
        .map(ToOwned::to_owned)
}

fn decode_word_usize(word: &[u8]) -> Option<usize> {
    if word.len() != 32 || word[..24].iter().any(|&b| b != 0) {
        return None;
    }

    let mut low = [0u8; 8];
    low.copy_from_slice(&word[24..32]);
    usize::try_from(u64::from_be_bytes(low)).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_hex_payload_returns_first_valid_hex_token() {
        let err = "rpc error (data: 0xdeadbeef1234), trailing";
        let payload = find_hex_payload(err).expect("must find hex payload");
        assert_eq!(payload, "0xdeadbeef1234");
    }

    #[test]
    fn decode_revert_detail_decodes_error_string() {
        let revert = encode_error_string_revert("oops");
        let err = format!("execution reverted: {revert}");

        let decoded = decode_revert_detail(&err);
        assert_eq!(decoded, "execution reverted: oops");
    }

    #[test]
    fn decode_revert_detail_decodes_panic_code() {
        let revert = encode_panic_revert(0x11);
        let err = format!("vm error with data {revert}");

        let decoded = decode_revert_detail(&err);
        assert_eq!(decoded, "execution panicked with code 0x11");
    }

    #[test]
    fn decode_revert_detail_falls_back_for_unknown_selector() {
        let err = "failed with data 0x12345678abcd";
        let decoded = decode_revert_detail(err);

        assert!(decoded.contains("selector=0x12345678"));
        assert!(decoded.contains("revert_data=0x12345678abcd"));
    }

    fn encode_error_string_revert(reason: &str) -> String {
        let reason_bytes = reason.as_bytes();
        let padded_len = reason_bytes.len().div_ceil(32) * 32;

        let mut data = Vec::with_capacity(4 + 32 + 32 + padded_len);
        data.extend_from_slice(&ERROR_STRING_SELECTOR);

        let mut offset = [0u8; 32];
        offset[31] = 32;
        data.extend_from_slice(&offset);

        let mut len = [0u8; 32];
        len[24..32].copy_from_slice(&(reason_bytes.len() as u64).to_be_bytes());
        data.extend_from_slice(&len);

        let mut payload = vec![0u8; padded_len];
        payload[..reason_bytes.len()].copy_from_slice(reason_bytes);
        data.extend_from_slice(&payload);

        format!("0x{}", hex::encode(data))
    }

    fn encode_panic_revert(code: u64) -> String {
        let mut data = Vec::with_capacity(4 + 32);
        data.extend_from_slice(&PANIC_SELECTOR);

        let mut word = [0u8; 32];
        word[24..32].copy_from_slice(&code.to_be_bytes());
        data.extend_from_slice(&word);

        format!("0x{}", hex::encode(data))
    }
}

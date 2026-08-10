//! Converts an aggregated ARM transaction into `execute` calldata for the
//! protocol adapter — the worked example of the `From<Transaction>` conversion
//! in `src/conversion.rs`.
//!
//! ```text
//! cargo run -p anoma-pa-evm-bindings --example execute_calldata -- <transaction.bin> [kind_table.json]
//! ```
//!
//! The input is the bincode encoding of an `anoma-rm-risc0` `Transaction`, as
//! written by that repo's `anoma-rm-risc0-tx-gen`. It must come from the arm
//! revision this crate pins: bincode carries no schema, so a transaction from
//! another revision either fails to decode or decodes into the wrong shape.
//!
//! The kind table is optional and only feeds `Transaction::verify`, which
//! checks the transaction against the process-global table. Pass the file the
//! transaction was proven against to verify before spending gas; omit it to
//! convert unverified.

use alloy::hex;
use alloy::sol_types::SolCall;
use anoma_pa_evm_bindings::generated::protocol_adapter::{IProtocolAdapter, ProtocolAdapter};
use anoma_rm_risc0::Digest;
use anoma_rm_risc0::constants::{
    BATCH_AGGREGATION_EVM_VK, COMPLIANCE_VK, init_kind_table_from_file,
};
use anoma_rm_risc0::transaction::Transaction;
use anyhow::Context;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let transaction_path = PathBuf::from(
        args.next()
            .context("usage: execute_calldata <transaction.bin> [kind_table.json]")?,
    );
    let kind_table_path = args.next().map(PathBuf::from);

    let encoded = std::fs::read(&transaction_path)
        .with_context(|| format!("failed to read {}", transaction_path.display()))?;
    let transaction: Transaction = bincode::deserialize(&encoded).context(
        "failed to decode the transaction — check that it comes from the pinned arm revision",
    )?;

    match kind_table_path {
        Some(path) => {
            init_kind_table_from_file(&path)
                .map_err(|err| anyhow::anyhow!("failed to load the kind table: {err:?}"))?;
            transaction
                .verify()
                .map_err(|err| anyhow::anyhow!("the transaction failed verification: {err:?}"))?;
            println!("verified against {}", path.display());
        }
        None => println!("not verified (no kind table given)"),
    }

    let instance = transaction
        .aggregation
        .as_ref()
        .context("the transaction carries no aggregation proof")?
        .instance
        .clone();

    let transaction = IProtocolAdapter::Transaction::from(transaction);
    let calldata = ProtocolAdapter::executeCall {
        transaction: transaction.clone(),
    }
    .abi_encode();

    let output_path = transaction_path.with_extension("calldata.txt");
    std::fs::write(&output_path, format!("0x{}", hex::encode(&calldata)))
        .with_context(|| format!("failed to write {}", output_path.display()))?;

    let seal = transaction.aggregationProof;
    println!("\nthe protocol adapter must agree on");
    println!(
        "  batch aggregation vk:  {}",
        hex32(&BATCH_AGGREGATION_EVM_VK)
    );
    println!("  compliance vk:         {}", hex32(&COMPLIANCE_VK));
    println!(
        "  kind table commitment: {}",
        hex32(&instance.kind_table_commitment)
    );
    println!("  seal selector:         0x{}", hex::encode(&seal[..4]));

    println!("\nthe transaction");
    println!("  actions:               {}", transaction.actions.len());
    for (index, action) in transaction.actions.iter().enumerate() {
        println!(
            "  action {index}:              {} consumed, {} created",
            action.consumed.len(),
            action.created.len()
        );
    }
    println!("  calldata bytes:        {}", calldata.len());

    println!("\nwrote {}", output_path.display());

    Ok(())
}

fn hex32(digest: &Digest) -> String {
    format!("0x{}", hex::encode(digest.as_bytes()))
}

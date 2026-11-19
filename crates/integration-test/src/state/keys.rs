pub const KEY_CHAIN_ID: &str = "evm.chain.id";
pub const KEY_CHAIN_NAME: &str = "evm.chain.name";

#[inline]
pub fn chain_id_key() -> &'static str {
    KEY_CHAIN_ID
}

#[inline]
pub fn chain_name_key() -> &'static str {
    KEY_CHAIN_NAME
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_state_keys_are_stable() {
        assert_eq!(chain_id_key(), "evm.chain.id");
        assert_eq!(chain_name_key(), "evm.chain.name");
    }
}

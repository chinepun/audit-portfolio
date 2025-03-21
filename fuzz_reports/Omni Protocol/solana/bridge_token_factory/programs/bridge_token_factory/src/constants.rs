use anchor_lang::prelude::*;

#[constant]
pub const CONFIG_SEED: &[u8] = b"config";

#[constant]
pub const AUTHORITY_SEED: &[u8] = b"authority";

#[constant]
pub const VAULT_SEED: &[u8] = b"vault";

#[constant]
pub const SOL_VAULT_SEED: &[u8] = b"sol_vault";

#[constant]
pub const USED_NONCES_SEED: &[u8] = b"used_nonces";

#[constant]
pub const WRAPPED_MINT_SEED: &[u8] = b"wrapped_mint";

#[constant]
pub const USED_NONCES_PER_ACCOUNT: u32 = 1024;

#[constant]
pub const USED_NONCES_ACCOUNT_SIZE: u32 = 8 + (USED_NONCES_PER_ACCOUNT + 7) / 8;

#[constant]
pub const SOLANA_OMNI_BRIDGE_CHAIN_ID: u8 = 2;


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_USED_NONCES_ACCOUNT_SIZE() {
        assert_eq!(
            USED_NONCES_ACCOUNT_SIZE, 
            136
        );
    }
}
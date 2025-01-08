use anchor_lang::prelude::*;

#[derive(Default, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub struct WormholeBumps {
    pub bridge: u8,
    pub fee_collector: u8,
    pub sequence: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct ConfigBumps {
    pub config: u8,
    pub authority: u8,
    pub sol_vault: u8,
    pub wormhole: WormholeBumps,
}

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub max_used_nonce: u64,
    pub derived_near_bridge_address: [u8; 64],
    pub bumps: ConfigBumps,
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test_CONFIG_SIZE() {
//         assert_eq!(
//             std::mem::size_of::<Config>(), 
//             Config::INIT_SPACE,
//             // 32 + 8 + 64 + 
//             // (
//             //     1 + 1 + 1 +
//             //     (
//             //         1 + 1 + 1
//             //     )
//             // )
//         );
//     }
// }

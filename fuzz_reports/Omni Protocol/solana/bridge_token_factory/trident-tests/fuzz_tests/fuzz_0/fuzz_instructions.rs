use trident_client::fuzzing::*;


use crate::instructions::{
    initialize::Initialize,
    deploy_token::DeployToken,
    init_transfer::InitTransfer,
    finalize_transfer::FinalizeTransfer,
    finalize_transfer_sol::FinalizeTransferSol,
    log_metadata::LogMetadata,
    init_transfer_sol::InitTransferSol
};


/// FuzzInstruction contains all available Instructions.
/// Below, the instruction arguments (accounts and data) are defined.
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
pub enum FuzzInstruction {
    Initialize(Initialize),
    DeployToken(DeployToken),
    FinalizeTransfer(FinalizeTransfer),
    FinalizeTransferSol(FinalizeTransferSol),
    LogMetadata(LogMetadata),
    InitTransfer(InitTransfer),
    InitTransferSol(InitTransferSol),
}

/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    pub admin: AccountsStorage<KeypairStore>,
    // pub config_pda: AccountsStorage<ProgramStore>,
    pub authority_pda: AccountsStorage<PdaStore>,
    pub sol_vault_pda: AccountsStorage<PdaStore>,
    // pub vault_pda: AccountsStorage<PdaStore> 
    // pub mint_pda: AccountsStorage<PdaStore>,
    pub random_mint: AccountsStorage<MintStore>,
    pub from: AccountsStorage<TokenStore>,
    pub user: AccountsStorage<KeypairStore>,
    // pub wormhole_bridge_pda: AccountsStorage<ProgramStore>,
    // pub wormhole_fee_collector_pda: AccountsStorage<ProgramStore>,
    pub token_account: AccountsStorage<TokenStore>,
    // pub wormhole_sequence_pda: AccountsStorage<ProgramStore>,
    // pub used_nonces: AccountsStorage<ProgramStore>,
    pub wormhole_message: AccountsStorage<KeypairStore>,
    pub recipient: AccountsStorage<KeypairStore>,
    pub payer: AccountsStorage<KeypairStore>,
    // pub metadata_pda: AccountsStorage<ProgramStore>,
    // pub _clock: AccountsStorage<ProgramStore>,
    // pub _rent: AccountsStorage<ProgramStore>,
    // pub _system_program: AccountsStorage<ProgramStore>,
    // pub _wormhole_program: AccountsStorage<ProgramStore>,
    // pub _program: AccountsStorage<ProgramStore>,
}

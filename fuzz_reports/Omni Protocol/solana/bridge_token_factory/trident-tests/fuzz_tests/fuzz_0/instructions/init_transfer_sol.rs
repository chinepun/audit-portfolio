use trident_client::fuzzing::*;
use bridge_token_factory::constants::{CONFIG_SEED, AUTHORITY_SEED, SOL_VAULT_SEED};
use wormhole_anchor_sdk::wormhole::{self, };
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use crate::fuzz_instructions::{FuzzAccounts,};
use solana_sdk::sysvar::{ SysvarId};
use crate::constants::PUBKEY_BYTES;

#[derive(Arbitrary, Debug)]
pub struct InitTransferSol {
    pub accounts: InitTransferSolAccounts,
    pub data: InitTransferSolData,
}

#[derive(Arbitrary, Debug)]
pub struct InitTransferSolAccounts {
    pub authority: AccountId,
    pub sol_vault: AccountId,
    // pub user: AccountId,
    // pub config: AccountId,
    // pub wormhole_bridge: AccountId,
    // pub wormhole_fee_collector: AccountId,
    // pub wormhole_sequence: AccountId,
    pub wormhole_message: AccountId,
    pub payer: AccountId,
    // pub _clock: AccountId,
    // pub _rent: AccountId,
    // pub _wormhole_program: AccountId,
    // pub _system_program: AccountId,
}

#[derive(Arbitrary, Debug)]
pub struct InitTransferSolData {
    pub amount: u128,
    pub recipient: String,
    pub fee: u128,
    pub native_fee: u64,
}

impl IxOps for InitTransferSol {
    type IxData = bridge_token_factory::instruction::InitTransferSol;
    type IxAccounts = FuzzAccounts;

    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        bridge_token_factory::ID
    }

    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {

        let data = bridge_token_factory::instruction::InitTransferSol {
            payload: bridge_token_factory::state::message::init_transfer::InitTransferPayload {
                amount: self.data.amount,
                recipient: self.data.recipient.clone(),
                fee: self.data.fee,
                native_fee: self.data.native_fee,
            }
        };

        Ok(data)
    }

    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let (config, _) = Pubkey::find_program_address(
            &[CONFIG_SEED],
            &bridge_token_factory::ID
        );

        let authority = fuzz_accounts.authority_pda.get_or_create_account(
            self.accounts.authority,
            client,
            &[AUTHORITY_SEED],
            &bridge_token_factory::ID
        );
        let sol_vault = fuzz_accounts.sol_vault_pda.get_or_create_account(
            self.accounts.sol_vault,
            client,
            &[SOL_VAULT_SEED],
            &bridge_token_factory::ID
        );

        let (wormhole_bridge, _) = Pubkey::find_program_address(
            &[wormhole::BridgeData::SEED_PREFIX],
            &wormhole_anchor_sdk::wormhole::program::id()
        );
        // let bridge_data = wormhole::BridgeData{
        //     guardian_set_index: 1,
        //     last_lamports: LAMPORTS_PER_SOL,
        //     config: wormhole::BridgeConfig{
        //         guardian_set_expiration_time: 2,
        //         fee: 2
        //     }
        // };

        // let mint_data = spl_token::state::Mint {
        //     mint_authority: COption::Some(authority),
        //     supply: u64::MAX,
        //     decimals: DECIMALS,
        //     is_initialized: true,
        //     freeze_authority: COption::None,
        // };

        // let mut data = vec![0; spl_token::state::Mint::LEN];
        // spl_token::state::Mint::pack(mint_data, &mut data);

        // client.set_account_custom(
        //     //mint_pubkey,
        //     AccountSharedData::from(Account {
        //         lamports: 1_000,
        //         data,
        //         owner: spl_token::id(),
        //         executable: false,
        //         rent_epoch: 0,
        //     })
        // )

        
        let (wormhole_fee_collector, _) = Pubkey::find_program_address(
            &[wormhole::FeeCollector::SEED_PREFIX],
            &wormhole_anchor_sdk::wormhole::program::id()
        );

        let (wormhole_sequence, _) = Pubkey::find_program_address(
            &[wormhole::SequenceTracker::SEED_PREFIX, config.as_ref()],
            &wormhole_anchor_sdk::wormhole::program::id(),
        );
        // let wormhole_message = fuzz_accounts.wormhole_message.get_or_create_account(
        //     self.accounts.wormhole_message,
        //     client,
        //     0 * LAMPORTS_PER_SOL
        // );
        let wormhole_message = Keypair::new();
        let payer = fuzz_accounts.payer.get_or_create_account(
            self.accounts.payer,
            client,
            10 * LAMPORTS_PER_SOL
        );
        let signers = vec![wormhole_message.clone(), payer.clone()];
        let acc_meta = bridge_token_factory::accounts::InitTransferSol {
            authority,
            sol_vault,
            user: config,
            wormhole: bridge_token_factory::accounts::WormholeCPI{
                config: config,
                bridge: wormhole_bridge,
                fee_collector: wormhole_fee_collector,
                sequence: wormhole_sequence,
                message: wormhole_message.pubkey(),
                payer: payer.pubkey(),
                clock: solana_sdk::sysvar::clock::Clock::id(),
                rent: solana_sdk::sysvar::rent::Rent::id(),
                system_program: solana_sdk::system_program::ID,
                wormhole_program: wormhole_anchor_sdk::wormhole::program::ID
            },
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }

    fn check(
        &self,
        _pre_ix: &[SnapshotAccount],
        _post_ix: &[SnapshotAccount],
        _ix_data: Self::IxData,
    ) -> Result<(), FuzzingError> {        
        Ok(())
    }

    #[allow(unused_variables)]
    fn tx_error_handler(
        &self,
        e: FuzzClientErrorWithOrigin,
        _ix_data: Self::IxData,
        _pre_ix_acc_infos: &[SnapshotAccount],
    ) -> Result<(), FuzzClientErrorWithOrigin> {


        eprintln!("Pubkey = {:?} ", PUBKEY_BYTES);

        Err(e)
    }
}

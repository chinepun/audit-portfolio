use trident_client::fuzzing::*;

use bridge_token_factory::constants::{CONFIG_SEED, AUTHORITY_SEED, SOL_VAULT_SEED};
use wormhole_anchor_sdk::wormhole::{self, };
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use crate::fuzz_instructions::{FuzzAccounts,};
use solana_sdk::sysvar::{ SysvarId};
use crate::constants::PUBKEY_BYTES;
#[derive(Arbitrary, Debug)]
pub struct Initialize {
    pub accounts: InitializeAccounts,
    pub data: InitializeData,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeAccounts {
    // pub config: AccountId,
    pub authority: AccountId,
    pub sol_vault: AccountId,
    // pub wormhole_bridge: AccountId,
    // pub wormhole_fee_collector: AccountId,
    // pub wormhole_sequence: AccountId,
    pub wormhole_message: AccountId,
    pub payer: AccountId,
    // pub _clock: AccountId,
    // pub _rent: AccountId,
    // pub _system_program: AccountId,
    // pub _wormhole_program: AccountId,
    // pub _program: AccountId,
}

#[derive(Debug)]
pub struct InitializeData {
    pub admin: AccountId,
    pub derived_near_bridge_address: [u8; 64],
}
impl<'a> Arbitrary<'a> for InitializeData
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        // Obtain AccountID
        let admin = AccountId::arbitrary(u)?;
        let derived_near_bridge_address = PUBKEY_BYTES;
        Ok(InitializeData { admin, derived_near_bridge_address })
    }
}

impl IxOps for Initialize {
    type IxData = bridge_token_factory::instruction::Initialize;
    type IxAccounts = FuzzAccounts;

    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        bridge_token_factory::ID
    }

    fn get_data(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let admin = fuzz_accounts.admin.get_or_create_account(
            self.data.admin,
            client,
            10 * LAMPORTS_PER_SOL
        );

        let data = bridge_token_factory::instruction::Initialize {
            admin: admin.pubkey(),
            derived_near_bridge_address: self.data.derived_near_bridge_address,
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
        // let mut data: Vec<u8> = vec![];
        // bridge_data.try_serialize(&mut data).unwrap();
        
        // client.set_account_custom(
        //     &wormhole_bridge,
        //     &AccountSharedData::create(
        //         1 * LAMPORTS_PER_SOL,
        //         data,
        //         &wormhole_anchor_sdk::wormhole::program::id(),
        //         false,
        //         0,
        //     )
        // );

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
        
        let acc_meta = bridge_token_factory::accounts::Initialize {
            config: config,
            authority: authority,
            sol_vault: sol_vault,
            wormhole_bridge: wormhole_bridge,
            wormhole_fee_collector: wormhole_fee_collector,
            wormhole_sequence: wormhole_sequence,
            wormhole_message: wormhole_message.pubkey(),
            payer: payer.pubkey(),
            clock: solana_sdk::sysvar::clock::Clock::id(),
            rent: solana_sdk::sysvar::rent::Rent::id(),
            system_program: solana_sdk::system_program::ID,
            wormhole_program: wormhole_anchor_sdk::wormhole::program::ID,
            // program: bridge_token_factory::ID
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
        ix_data: Self::IxData,
        pre_ix_acc_infos: &[SnapshotAccount],
    ) -> Result<(), FuzzClientErrorWithOrigin> {
        // if pre_ix_acc_infos.wormhole == solana_sdk::pubkey::Pubkey::default() {
            eprintln!("error {}", pre_ix_acc_infos[4].owner());
        // }
        Err(e)
    }
}

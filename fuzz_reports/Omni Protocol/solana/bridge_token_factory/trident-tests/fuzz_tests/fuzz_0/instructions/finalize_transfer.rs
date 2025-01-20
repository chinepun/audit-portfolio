use trident_client::fuzzing::*;
use bridge_token_factory::constants::{CONFIG_SEED, AUTHORITY_SEED};
use wormhole_anchor_sdk::wormhole::{self, };
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use crate::fuzz_instructions::{FuzzAccounts,};
use solana_sdk::sysvar::{ SysvarId};
use bridge_token_factory::constants::VAULT_SEED;
use crate::constants::{SIGNATURE_BYTES};
use bridge_token_factory::state::message::SignedPayload;
use bridge_token_factory::constants::USED_NONCES_SEED;
use bridge_token_factory::constants::USED_NONCES_PER_ACCOUNT;
#[derive(Arbitrary, Debug)]
pub struct FinalizeTransfer {
    pub accounts: FinalizeTransferAccounts,
    pub data: FinalizeTransferData,
}

#[derive(Arbitrary, Debug)]
pub struct FinalizeTransferAccounts {
    // pub used_nonces: AccountId,
    pub authority: AccountId,
    pub recipient: AccountId,
    pub mint: AccountId,
    // pub vault: AccountId,
    pub token_account: AccountId,
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
    // pub associated_token_program: AccountId,
    // pub token_program: AccountId,
}

#[derive(Debug)]
pub struct FinalizeTransferData {
    pub destination_nonce: u64,
    pub transfer_id: bridge_token_factory::state::message::TransferId,
    pub amount: u128,
    pub fee_recipient: Option<String>,
}

impl<'a> Arbitrary<'a> for FinalizeTransferData
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {

        let origin_chain = u.int_in_range(1..=10)?;
        let origin_nonce = u.int_in_range(1..=1_000_000)?;

        let transfer_id = bridge_token_factory::state::message::TransferId{ origin_chain, origin_nonce };

        Ok(FinalizeTransferData {
            destination_nonce: u64::arbitrary(u)?,
            transfer_id,
            amount: u128::arbitrary(u)?,
            fee_recipient: String::arbitrary(u).ok(),
        })
    }
}

impl IxOps for FinalizeTransfer
{
    type IxData = bridge_token_factory::instruction::FinalizeTransfer;
    type IxAccounts = FuzzAccounts;

    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        bridge_token_factory::ID
    }

    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {

        let data = bridge_token_factory::instruction::FinalizeTransfer {
            data: SignedPayload {
                payload: bridge_token_factory::state::message::finalize_transfer::FinalizeTransferPayload {
                    destination_nonce: self.data.destination_nonce,
                    transfer_id: self.data.transfer_id.clone(),
                    amount: self.data.amount,
                    fee_recipient: self.data.fee_recipient.clone(),
                },
                signature: SIGNATURE_BYTES,
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

        let recipient = fuzz_accounts.recipient.get_or_create_account(
            self.accounts.recipient,
            client,
            5 * LAMPORTS_PER_SOL,
        );

        let mint = fuzz_accounts.random_mint.get_or_create_account(
            self.accounts.mint,
            client,
            9,
            &authority,
            None,
        );

        // let destination_nonce = 1;// destination_nonce(1024) / USED_NONCES_PER_ACCOUNT;

        let (used_nonces, _) = Pubkey::find_program_address(
            &[USED_NONCES_SEED, &(self.data.destination_nonce / USED_NONCES_PER_ACCOUNT as u64).to_le_bytes()],
            &bridge_token_factory::ID
        );

        let (vault, _) = Pubkey::find_program_address(
            &[VAULT_SEED, mint.as_ref()],
            &bridge_token_factory::ID
        );

        let token_account = fuzz_accounts.token_account.get_or_create_account(
            self.accounts.token_account,
            client,
            mint,
            recipient.pubkey(),
            1_000,
            None,
            None,
            0,
            None,
        );

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
        let (wormhole_bridge, _) = Pubkey::find_program_address(
            &[wormhole::BridgeData::SEED_PREFIX],
            &wormhole_anchor_sdk::wormhole::program::id()
        );

        let signers = vec![wormhole_message.clone(), payer.clone()];
        let acc_meta = bridge_token_factory::accounts::FinalizeTransfer {
            config: config,
            used_nonces: used_nonces,
            authority: authority,
            recipient: recipient.pubkey(),
            mint: mint,
            vault: Some(vault),
            token_account: token_account,
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
            associated_token_program: anchor_spl::associated_token::ID,
            system_program: solana_sdk::system_program::ID,
            token_program: anchor_spl::token::ID,
        }.to_account_metas(None);

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


        // eprintln!("Pubkey = {:?} ", PUBKEY_BYTES);

        Err(e)
    }

}
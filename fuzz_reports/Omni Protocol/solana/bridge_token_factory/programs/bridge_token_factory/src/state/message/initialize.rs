
use std::io::{BufWriter};
use anchor_lang::prelude::*;
use crate::{error::ErrorCode};
use super::{
    OutgoingMessageType, Payload, DEFAULT_SERIALIZER_CAPACITY,
};
#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct InitializePayload {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub admin: Pubkey,
    pub wormhole_sequence: Pubkey,
}

impl Payload for InitializePayload {
    type AdditionalParams = ();

    fn serialize_for_near(&self, _params: Self::AdditionalParams) -> Result<Vec<u8>> {
        let mut writer = BufWriter::new(Vec::with_capacity(DEFAULT_SERIALIZER_CAPACITY));
        // 0. Message type
        OutgoingMessageType::Initialize.serialize(&mut writer)?;
        // 1 config
        // writer.write(&[SOLANA_OMNI_BRIDGE_CHAIN_ID])?;
        self.config.serialize(&mut writer)?;
        // 2 authority
        // self.authority.serialize(&mut writer)?;
        // // 3 admin
        // self.admin.serialize(&mut writer)?;
        // // 4 wormhole sequence
        // self.wormhole_sequence.serialize(&mut writer)?;

        writer
            .into_inner()
            .map_err(|_| error!(ErrorCode::InvalidArgs))
        
    }
}

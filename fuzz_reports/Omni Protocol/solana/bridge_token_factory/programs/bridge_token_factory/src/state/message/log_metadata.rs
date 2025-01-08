use std::io::{BufWriter, Write};

use super::{OutgoingMessageType, Payload, DEFAULT_SERIALIZER_CAPACITY};
use crate::{constants::SOLANA_OMNI_BRIDGE_CHAIN_ID, error::ErrorCode};
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct LogMetadataPayload {
    pub token: Pubkey,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

impl Payload for LogMetadataPayload {
    type AdditionalParams = ();

    fn serialize_for_near(&self, _params: Self::AdditionalParams) -> Result<Vec<u8>> {
        let mut writer = BufWriter::new(Vec::with_capacity(DEFAULT_SERIALIZER_CAPACITY));
        // 0. Message type
        OutgoingMessageType::LogMetadata.serialize(&mut writer)?;
        // 1. token
        writer.write(&[SOLANA_OMNI_BRIDGE_CHAIN_ID])?;
        self.token.serialize(&mut writer)?;
        // 2. name
        self.name.serialize(&mut writer)?;
        // 3. symbol
        self.symbol.serialize(&mut writer)?;
        // 4. decimals
        writer.write(&[self.decimals])?;

        writer
            .into_inner()
            .map_err(|_| error!(ErrorCode::InvalidArgs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use borsh::{to_vec, BorshDeserialize};
    use std::io::Cursor;
    use crate::borsh::maybestd::io;
    struct CustomReader {
        data: Vec<u8>,
        read_index: usize,
    }
    
    impl io::Read for CustomReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let len = buf.len().min(self.data.len() - self.read_index);
            buf[0..len].copy_from_slice(&self.data[self.read_index..self.read_index + len]);
            self.read_index += len;
            Ok(len)
        }
    }

    #[test]
    fn test_custom_reader_with_insufficient_data() {
        let s = LogMetadataPayload {
            token: Pubkey::default(),
            name: "name".to_string(),
            symbol: "symbol".to_string(),
            decimals: 0,
        };
        let mut bytes = to_vec(&s).unwrap();
        // bytes.pop().unwrap();
        let mut reader = CustomReader {
            data: bytes,
            read_index: 0,
        };
        let de: LogMetadataPayload = BorshDeserialize::deserialize_reader(&mut reader).unwrap();
        assert_eq!(de.token, s.token);
        assert_eq!(de.name, s.name);
        assert_eq!(de.symbol, s.symbol);
        assert_eq!(de.decimals, s.decimals);
        // assert_eq!(
        //     <Serializable as BorshDeserialize>::deserialize_reader(&mut reader)
        //         .unwrap_err()
        //         .to_string(),
        //     ERROR_UNEXPECTED_LENGTH_OF_INPUT
        // );

    }
}
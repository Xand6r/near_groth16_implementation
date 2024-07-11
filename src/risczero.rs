use ::easy_hasher::easy_hasher::raw_sha256;
use eth_encode_packed::{
    ethabi::{ethereum_types::U256, Address},
    SolidityDataType, TakeLastXBytes,
};

use easy_hasher::easy_hasher;

use crate::{constants::ZERO_BYTES_STRING, utils::get_zero_bytes};
#[derive(Debug)]
pub enum SystemExitCode {
    Halted,
    Paused,
    SystemSplit,
}

#[derive(Debug)]
pub struct SystemStateLib {
    pub tag_digest: Vec<u8>,
}

impl SystemStateLib {
    fn get_tag_digest() -> Vec<u8> {
        return easy_hasher::sha256(&"risc0.SystemState".to_string()).to_vec();
    }
}

#[derive(Debug)]
pub struct Output {
    pub journal_digest: Vec<u8>,
    pub assumptions_digest: Vec<u8>,
}

impl Output {
    fn digest(self) -> Vec<u8> {
        let tag_digest = easy_hasher::sha256(&"risc0.Output".to_string()).to_vec();

        let input = vec![
            SolidityDataType::Bytes(&tag_digest),
            SolidityDataType::Bytes(&self.journal_digest),
            SolidityDataType::Bytes(&self.assumptions_digest),
            SolidityDataType::NumberWithShift(U256::from(2) << 8, TakeLastXBytes(16)),
        ];
        let (bytes, _) = eth_encode_packed::abi::encode_packed(&input);
        let out = raw_sha256(bytes).to_vec();

        out
    }
}

#[derive(Debug)]
pub struct Receipt {
    pub seal: Vec<u8>,
    pub claim: ReceiptClaim,
}

#[derive(Debug)]
pub struct ExitCode {
    pub system: SystemExitCode,
    pub user: u8,
}

#[derive(Debug)]
pub struct ReceiptClaim {
    pub pre_state_digest: Vec<u8>,
    pub post_state_digest: Vec<u8>,
    pub exit_code: ExitCode,
    pub input: Vec<u8>,
    pub output: Vec<u8>,
}

impl ReceiptClaim {
    pub fn digest(self) -> Vec<u8> {
        let tag_digest = easy_hasher::sha256(&"risc0.ReceiptClaim".to_string()).to_vec();

        let input = vec![
            SolidityDataType::Bytes(&tag_digest),
            SolidityDataType::Bytes(&self.input),
            SolidityDataType::Bytes(&self.pre_state_digest),
            SolidityDataType::Bytes(&self.post_state_digest),
            SolidityDataType::Bytes(&self.output),
            SolidityDataType::NumberWithShift(
                U256::from(self.exit_code.system as u16) << 24,
                TakeLastXBytes(32),
            ),
            SolidityDataType::NumberWithShift(
                U256::from(self.exit_code.user) << 24,
                TakeLastXBytes(32),
            ),
            SolidityDataType::NumberWithShift(U256::from(4) << 8, TakeLastXBytes(16)),
        ];
        let (bytes, _) = eth_encode_packed::abi::encode_packed(&input);
        let out = raw_sha256(bytes).to_vec();

        out
    }

    pub fn ok(image_id: Vec<u8>, journal_digest: Vec<u8>) -> Self{
        let (system_state_zero_digest, _) = Self::system_state_zero_digest();
        ReceiptClaim{
            pre_state_digest: image_id,
            post_state_digest: system_state_zero_digest,
            exit_code: ExitCode {
                system: SystemExitCode::Halted,
                user: 0,
            },
            input: hex::decode(ZERO_BYTES_STRING).unwrap(),
            output: Output {
                journal_digest: journal_digest.clone(),
                assumptions_digest: hex::decode(
                    ZERO_BYTES_STRING,
                )
                .unwrap(),
            }
            .digest(),
        }
    }

    pub fn system_state_zero_digest() -> (Vec<u8>, String) {
        let zero_bytes = get_zero_bytes();
        let system_state_tag_digest = SystemStateLib::get_tag_digest();

        let input = vec![
            SolidityDataType::Bytes(&system_state_tag_digest),
            SolidityDataType::Bytes(&zero_bytes),
            SolidityDataType::NumberWithShift(U256::from(0), TakeLastXBytes(32)),
            SolidityDataType::NumberWithShift(U256::from(1) << 8, TakeLastXBytes(16)),
        ];

        let response = eth_encode_packed::abi::encode_packed(&input);
        
        response
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::{IMAGE_ID_STRING, JOURNAL_OUTPUT_STRING, SEAL_STRING};

    use self::easy_hasher::raw_sha256;
    use eth_encode_packed::ethabi::{encode, Token};

    use super::*;

    #[test]
    fn test_output_digest() {
        let journal_output =
            hex::decode("4f8ad5ce1d0fc577d04d618880b8c77c9aced63740ca43b708f32425a95b11b7")
                .unwrap();

        let journal_digest = raw_sha256(encode(&[Token::Bytes(journal_output)])).to_vec();

        let output = Output {
            journal_digest: journal_digest.clone(),
            assumptions_digest: hex::decode(
                "0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
        };
        let output_digest = output.digest();
        let expected_output_digest =
            "7e76559b3f58036f3e6d302930e1eccda6d667425fe3c2d9d69f913204e04ec2";

        assert_eq!(hex::encode(output_digest), expected_output_digest);
    }

    #[test]
    fn test_get_system_lib_digest() {
        let expected_digest = "206115a847207c0892e0c0547225df31d02a96eeb395670c31112dff90b421d6";

        let system_state_tag_digest = SystemStateLib::get_tag_digest();
        let system_state_tag_digest = hex::encode(system_state_tag_digest);

        assert_eq!(expected_digest, system_state_tag_digest, "INVALID_SYSTEM_LIB_DIGEST");
    }
    
    #[test]
    fn test_system_state_zero_digest(){
        let expected_state_zero_digest = "206115a847207c0892e0c0547225df31d02a96eeb395670c31112dff90b421d60000000000000000000000000000000000000000000000000000000000000000000000000100".to_string();
        let (_, state_zero_digest) = ReceiptClaim::system_state_zero_digest();
        
        assert_eq!(expected_state_zero_digest, state_zero_digest);
    }


    #[test]
    fn test_receipt_digest() {
        let image_id =
            hex::decode(IMAGE_ID_STRING)
                .unwrap();
        let seal = hex::decode(SEAL_STRING).unwrap();
        let journal_output =
            hex::decode(JOURNAL_OUTPUT_STRING)
                .unwrap();

        let journal_digest = raw_sha256(encode(&[Token::Bytes(journal_output)])).to_vec();

        let receipt = Receipt {
            seal,
            claim: ReceiptClaim::ok(image_id, journal_digest)            
        };
        let digest = receipt.claim.digest();

        let expected_digest = "95a8626e4a1d767960de44a419a9b6771a2a7449ed7594c68ea527c49f152b87";
        assert_eq!(hex::encode(digest), expected_digest);
    }
}

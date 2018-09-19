extern crate prost;

pub mod ed25519msg;
pub mod heartbeat;
pub mod poisonpill;
pub mod proposal;
pub mod vote;

use bytes::BufMut;
use signatory::{ed25519::Ed25519Signature, Signature};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone, PartialEq, Message)]
pub struct PartsSetHeader {
    #[prost(sint64, tag = "1")]
    total: i64,
    #[prost(bytes, tag = "2")]
    hash: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BlockID {
    #[prost(bytes, tag = "1")]
    hash: Vec<u8>,
    #[prost(message, tag = "2")]
    parts_header: Option<PartsSetHeader>,
}

#[derive(Clone, PartialEq, Message)]
pub struct Time {
    #[prost(sfixed64, tag = "1")]
    pub seconds: i64,
    #[prost(sfixed32, tag = "2")]
    pub nanos: i32,
}

/// Converts `Time` to a `SystemTime`.
impl From<Time> for SystemTime {
    fn from(time: Time) -> SystemTime {
        if time.seconds >= 0 {
            UNIX_EPOCH + Duration::new(time.seconds as u64, time.nanos as u32)
        } else {
            UNIX_EPOCH - Duration::new(time.seconds as u64, time.nanos as u32)
        }
    }
}

pub trait TendermintSignable {
    fn sign_bytes<B>(&self, sign_bytes: &mut B) -> Result<bool, prost::EncodeError>
    where
        B: BufMut;
    fn set_signature(&mut self, sig: &Ed25519Signature);
}

pub use self::ed25519msg::PubKeyMsg;
pub use self::ed25519msg::AMINO_NAME as PUBKEY_AMINO_NAME;
pub use self::heartbeat::SignHeartbeatMsg;
pub use self::heartbeat::AMINO_NAME as HEARTBEAT_AMINO_NAME;
pub use self::poisonpill::PoisonPillMsg;
pub use self::poisonpill::AMINO_NAME as POISON_PILL_AMINO_NAME;
pub use self::proposal::SignProposalMsg;
pub use self::proposal::AMINO_NAME as PROPOSAL_AMINO_NAME;
pub use self::vote::SignVoteMsg;
pub use self::vote::AMINO_NAME as VOTE_AMINO_NAME;

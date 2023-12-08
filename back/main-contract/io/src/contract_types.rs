use gear_lib_old::non_fungible_token::token::TokenMetadata;
use gstd::{ActorId, Vec};

pub type NftContractId = ActorId;
pub type UserId = ActorId;
pub type MatchId = ActorId;
pub type TransactionId = u64;
pub type GameId = usize;
pub type NFTPrice = u128;
pub type DataFromContract = Vec<(ActorId, Vec<TokenMetadata>)>;
pub type Power = u8;

pub const ONE_TVARA_VALUE: u128 = 1000000000000;
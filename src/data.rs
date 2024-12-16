use crate::MARKET_DATA;
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{cell::ValueError, storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use shared::market::{MarketId, MarketName};
use std::borrow::Cow;

#[derive(
    CandidType, Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq, PartialOrd, Ord,
)]
pub enum MarketData {
    None,
    V1(MarketDataV1),
}

#[derive(
    CandidType, Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq, PartialOrd, Ord,
)]
pub struct MarketDataV1 {
    pub id: MarketId,
    pub name: MarketName,
}

impl Default for MarketData {
    fn default() -> Self {
        Self::None
    }
}

impl Storable for MarketData {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

pub(crate) fn update_market_data(id: MarketId, name: MarketName) -> Result<MarketData, ValueError> {
    let mut res: Result<MarketData, ValueError> = Ok(MarketData::None);

    MARKET_DATA.with_borrow_mut(|data| {
        res = data.set(MarketData::V1(MarketDataV1 { id, name }));
    });

    res
}

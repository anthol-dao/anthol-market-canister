use candid::Principal;
use data::MarketData;
use ic_cdk::storage;
use ic_cdk_macros::*;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, StableCell, StableLog,
};
use shared::{
    item::{ItemDataInMarket, ItemId},
    market::{MarketDataResponseWithItemGlances, MarketId, MarketInitArg, MarketName},
    store::StoreId,
    unit::Currency,
};
use std::{cell::RefCell, collections::VecDeque};

type Memory = VirtualMemory<DefaultMemoryImpl>;

mod data;
mod item;

#[cfg(feature = "canbench-rs")]
mod benches;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub(crate) static LOG: RefCell<StableLog<(), Memory, Memory>> = RefCell::new(
        StableLog::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        ).unwrap()
    );

    pub(crate) static MARKET_DATA: RefCell<StableCell<MarketData, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
            MarketData::default(),
        ).unwrap()
    );

    pub(crate) static ITEMS: RefCell<StableBTreeMap<(StoreId, ItemId), ItemDataInMarket, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
        )
    );

    pub(crate) static RECENT_ITEMS: RefCell<VecDeque<((StoreId, ItemId), ItemDataInMarket)>> = RefCell::new(VecDeque::with_capacity(30));
}

#[pre_upgrade]
fn pre_upgrade() {
    let recent_items = RECENT_ITEMS.with(|s| s.take());

    storage::stable_save((recent_items,)).expect("failed to save stable state");
}

#[post_upgrade]
fn post_upgrade() {
    let (recent_items,): (VecDeque<((StoreId, ItemId), ItemDataInMarket)>,) =
        storage::stable_restore().expect("failed to restore stable state");
    RECENT_ITEMS.with(|s| s.replace(recent_items));
}

#[init]
fn init_market(arg: Option<MarketInitArg>) {
    if let Some(arg) = arg {
        let _ = data::update_market_data(arg.id, arg.name);
    }
}

#[update]
fn update_market_data(caller: Principal, id: MarketId, name: MarketName) {
    let _ = data::update_market_data(id, name);
}

#[query]
fn get_recent_item_glances(
    caller: Principal,
    currency: Currency,
) -> MarketDataResponseWithItemGlances {
    item::get_recent_item_glances(currency)
}

#[update]
fn insert_items_to_market(caller: Principal, vec: Vec<((StoreId, ItemId), ItemDataInMarket)>) {
    item::insert_items(vec)
}

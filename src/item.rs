use crate::{data::MarketData, ITEMS, MARKET_DATA, RECENT_ITEMS};
use common::{
    item::ItemId,
    item::{get_item_glance_data, ItemDataInMarket, ItemGlanceData},
    market::{MarketDataResponseWithItemGlances, MarketId, MarketName},
    store::StoreId,
    unit::Currency,
};

pub fn get_recent_item_glances(currency: Currency) -> MarketDataResponseWithItemGlances {
    let (market_id, market_name) =
        MARKET_DATA.with(|market_data| match market_data.borrow().get() {
            MarketData::V1(market_data) => (market_data.id, market_data.name.clone()),
            MarketData::None => panic!("Market data not found"),
        });

    let vec: Vec<ItemGlanceData> = RECENT_ITEMS.with(|recent_items| {
        recent_items
            .borrow()
            .iter()
            .map(|((store_id, item_id), item)| {
                get_item_glance_data(store_id, item_id, item, &currency)
            })
            .collect()
    });

    MarketDataResponseWithItemGlances {
        id: market_id,
        name: market_name,
        items: vec,
    }
}

pub fn insert_items(vec: Vec<((StoreId, ItemId), ItemDataInMarket)>) {
    ITEMS.with(|items| {
        for (key, value) in vec.clone() {
            items.borrow_mut().insert(key, value);
        }
    });

    RECENT_ITEMS.with(|recent_items| {
        let mut recent_items = recent_items.borrow_mut();

        for (key, value) in vec {
            recent_items.push_front((key, value));

            if recent_items.len() > 24 {
                recent_items.pop_back();
            }
        }
    });
}

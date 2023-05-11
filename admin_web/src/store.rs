use std::{collections::HashMap, rc::Rc, future::Future};

use loquat_common::models::{FanSeries, FanSize};
use yew::platform::spawn_local;
use yewdux::prelude::*;

use crate::features::{fan_series::api::{index_fan_serieses, get_fan_series}, fan_size::api::get_fan_size};
#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct FanStore {
    pub fan_serieses: HashMap<String, FanSeries<()>>,
    pub fan_sizes: HashMap<String, FanSize<()>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]

pub enum FanStoreActions {
    IndexFanSeries,
    GetFanSeries(String),
    GetFanSize(String),
    InsertFanSeries(FanSeries<()>),
    InsertFanSize(FanSize<()>),
    InsertFanSeriesWithSizes(FanSeries<Vec<FanSize<()>>>),
    InsertFanSizeWithSeries(FanSize<FanSeries<()>>),
}

fn fetch_and_store<Fut, Resp>(fetching: Fut, action: impl Fn(Resp) -> FanStoreActions + 'static) 
where
Fut: Future<Output = Result<Resp, String>>+ 'static
{
    spawn_local(async move {
        if let Ok(json_resp) = fetching.await {
            let dispatch = Dispatch::<FanStore>::new();
            dispatch.apply(action(json_resp));
        }
    });
}

fn fetch_and_store_many<Fut, Resp: Sized>(fetching: Fut, action: impl Fn(Resp) -> FanStoreActions + 'static)
where
Fut: Future<Output = Result<Vec<Resp>, String>> + 'static
 {
    spawn_local(async move {
        if let Ok(json_resp) = fetching.await {
            let dispatch = Dispatch::<FanStore>::new();
            for item in json_resp{
                dispatch.apply(action(item));
            }
        }
    });
}

impl Reducer<FanStore> for FanStoreActions {
    fn apply(self, store: Rc<FanStore>) -> Rc<FanStore> {
        log::info!("Applying action {:#?}", &self);
        match self {
            FanStoreActions::IndexFanSeries => {
                fetch_and_store_many(index_fan_serieses(), FanStoreActions::InsertFanSeries);
                store
            },
            FanStoreActions::GetFanSeries(id) => {
                fetch_and_store(get_fan_series(id), FanStoreActions::InsertFanSeriesWithSizes);
                store
            },
            FanStoreActions::GetFanSize(id) => {
                fetch_and_store(get_fan_size(id), FanStoreActions::InsertFanSizeWithSeries);
                store
            },
            FanStoreActions::InsertFanSeries(fan_series) => {
                let mut fan_serieses = store.fan_serieses.clone();
                fan_serieses.insert(fan_series.id.clone(), fan_series);
                FanStore {
                    fan_serieses,
                    fan_sizes: store.fan_sizes.clone(),
                }
                .into()
            }
            FanStoreActions::InsertFanSize(fan_size) => {
                let mut fan_sizes = store.fan_sizes.clone();
                fan_sizes.insert(fan_size.id.clone(), fan_size);
                FanStore {
                    fan_serieses: store.fan_serieses.clone(),
                    fan_sizes,
                }
                .into()
            }
            FanStoreActions::InsertFanSeriesWithSizes(fan_series) => {
                let (flat_series, sizes) = fan_series.flatten();
                let store_with_series = FanStoreActions::InsertFanSeries(flat_series).apply(store);
                sizes.into_iter().fold(store_with_series, |sto, size| {
                    FanStoreActions::InsertFanSize(size).apply(sto)
                })
            }
            FanStoreActions::InsertFanSizeWithSeries(fan_size) => {
                let (size, series) = fan_size.flatten();
                let store_with_size = FanStoreActions::InsertFanSize(size).apply(store);
                FanStoreActions::InsertFanSeries(series).apply(store_with_size)
            }
        }
    }
}

pub mod selectors {
    use loquat_common::models::{FanSeries, FanSize};

    use super::FanStore;

    pub fn select_fan_sizes_for_fan_series_id(
        state: &FanStore,
        fan_series_id: &String,
    ) -> Vec<FanSize<()>> {
        state
            .fan_sizes
            .clone()
            .into_values()
            .filter(|fan_size| fan_size.fan_series_id == *fan_series_id)
            .collect()
    }

    pub fn select_fan_series_by_id(state: &FanStore, id: &String) -> Option<FanSeries<()>> {
        state.fan_serieses.get(id).cloned()
    }

    pub fn select_fan_size_by_id(state: &FanStore, id: &String) -> Option<FanSize<()>> {
        state.fan_sizes.get(id).cloned()
    }

    pub fn select_all_fan_series(state: &FanStore) -> Vec<FanSeries<()>> {
        state.fan_serieses.clone().into_values().collect()
    }
}

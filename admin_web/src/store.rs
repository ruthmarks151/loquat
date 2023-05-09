use std::{collections::HashMap, rc::Rc};

use loquat_common::models::{FanSeries, FanSize};
use yewdux::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct FanStore {
    pub fan_serieses: HashMap<String, FanSeries<()>>,
    pub fan_sizes: HashMap<String, FanSize<()>>,
}

pub enum FanStoreActions {
    InsertFanSeries(FanSeries<()>),
    InsertFanSize(FanSize<()>),
    InsertFanSeriesWithSizes(FanSeries<Vec<FanSize<()>>>),
    InsertFanSizeWithSeries(FanSize<FanSeries<()>>),
}

impl Reducer<FanStore> for FanStoreActions {
    fn apply(self, store: Rc<FanStore>) -> Rc<FanStore> {
        match self {
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

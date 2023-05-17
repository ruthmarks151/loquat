use std::{collections::HashMap, rc::Rc};

use loquat_common::models::FanSize;
use yewdux::{prelude, store::Reducer};

use crate::{api::store::ApiResponseAction, features::fan_series};

#[derive(Debug, Default, Clone, PartialEq, Eq, prelude::Store)]
pub struct Store {
    pub fan_sizes: HashMap<String, FanSize<()>>,
}

impl Reducer<Store> for ApiResponseAction {
    fn apply(self, mut og_state: Rc<Store>) -> Rc<Store> {
        let state: &mut Store = Rc::make_mut(&mut og_state);
        match self {
            ApiResponseAction::RecieveFanSeries(fan_series_with_size) => {
                let (_series, sizes) = fan_series_with_size.into();
                for size in sizes {
                    state.fan_sizes.insert(size.id.clone(), size);
                }
                og_state
            }
            ApiResponseAction::RecieveFanSize(size_with_fan_series) => {
                let (size, _series) = size_with_fan_series.into();
                state.fan_sizes.insert(size.id.clone(), size);
                og_state
            }
            ApiResponseAction::RecieveA1Report(report) => {
                let (size, _series) = report.fan_size.into();
                state.fan_sizes.insert(size.id.clone(), size);
                og_state
            }
            ApiResponseAction::RecieveFanSizes(fan_sizes) => {
                for fan_size in fan_sizes {
                    state.fan_sizes.insert(fan_size.id.clone(), fan_size);
                }
                og_state
            }
            _ => og_state,
        }
    }
}

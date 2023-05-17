use std::{collections::HashMap, rc::Rc};

use loquat_common::models::FanSeries;
use yewdux::{prelude, store::Reducer};

use crate::api::store::ApiResponseAction;

#[derive(Debug, Default, Clone, PartialEq, Eq, prelude::Store)]
pub struct Store {
    pub fan_serieses: HashMap<String, FanSeries<()>>,
}

impl Reducer<Store> for ApiResponseAction {
    fn apply(self, mut og_state: Rc<Store>) -> Rc<Store> {
        let state: &mut Store = Rc::make_mut(&mut og_state);

        match self {
            ApiResponseAction::RecieveFanSerieses(fan_serieses) => {
                for fan_series in fan_serieses {
                    state.fan_serieses.insert(fan_series.id.clone(), fan_series);
                }
                og_state
            }
            ApiResponseAction::RecieveFanSeries(fan_series) => {
                let (flat_series, _sizes) = fan_series.into();
                state
                    .fan_serieses
                    .insert(flat_series.id.clone(), flat_series);
                og_state
            }
            ApiResponseAction::RecieveFanSize(fan_size) => {
                let (_fan_size, flat_series) = fan_size.into();
                state
                    .fan_serieses
                    .insert(flat_series.id.clone(), flat_series);
                og_state
            }
            ApiResponseAction::RecieveA1Report(report) => {
                let flat_series = report.fan_size.fan_series;
                state
                    .fan_serieses
                    .insert(flat_series.id.clone(), flat_series);
                og_state
            }
        }
    }
}

use std::{collections::HashMap, rc::Rc};

use loquat_common::models::{A1Standard2010Report, FanSeries};
use yewdux::{prelude, store::Reducer};

use crate::api::store::ApiResponseAction;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Store {
    pub reports: HashMap<String, A1Standard2010Report<()>>,
}

impl prelude::Store for Store {
    fn new() -> Self {
        Self::default()
    }

    fn should_notify(&self, old: &Self) -> bool {
        !self.eq(old)
    }
}

impl Reducer<Store> for ApiResponseAction {
    fn apply(self, mut og_state: Rc<Store>) -> Rc<Store> {
        let state: &mut Store = Rc::make_mut(&mut og_state);

        match self {
            ApiResponseAction::RecieveA1Report(report) => {
                let (report, _fan_size) = report.into();
                state.reports.insert(report.id.clone(), report);
                log::info!("Inserted: {:#?}", state.reports);

                og_state
            }
            _ => og_state,
        }
    }
}

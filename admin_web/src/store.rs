use std::rc::Rc;

use loquat_common::models::{A1Standard2010Report, FanSeries, FanSize};
use yew::use_memo;
use yewdux::prelude::{use_store, Dispatch};

use crate::api::store::ApiResponseAction;

#[derive(Debug, Clone, PartialEq)]

pub struct AppStore {
    api: Rc<crate::api::store::Store>,
    a1_report: Rc<crate::features::a1_2010_report::Store>,
    fan_series: Rc<crate::features::fan_series::Store>,
    fan_size: Rc<crate::features::fan_size::Store>,
}

pub fn app_dispatch(action: ApiResponseAction) {
    log::info!("{:#?}", action);

    let a1_dispatch = Dispatch::<crate::features::a1_2010_report::Store>::new();
    let fan_series_dispatch = Dispatch::<crate::features::fan_series::Store>::new();
    let fan_size_dispatch = Dispatch::<crate::features::fan_size::Store>::new();

    a1_dispatch.apply(action.clone());
    fan_series_dispatch.apply(action.clone());
    fan_size_dispatch.apply(action);
}

#[yew::hook]
pub fn use_app_store_selector_with_deps<Func, Deps, Resp>(func: Func, deps: Deps) -> Rc<Resp>
where
    Func: Fn(&AppStore, &Deps) -> Resp,
    Deps: PartialEq + 'static,
    Resp: 'static,
{
    let (api_store, _) = use_store::<crate::api::store::Store>();
    let (a1_store, _) = use_store::<crate::features::a1_2010_report::Store>();
    let (fan_series, _) = use_store::<crate::features::fan_series::Store>();
    let (fan_size, _) = use_store::<crate::features::fan_size::Store>();

    let store = AppStore {
        api: Rc::clone(&api_store),
        a1_report: Rc::clone(&a1_store),
        fan_series: Rc::clone(&fan_series),
        fan_size: Rc::clone(&fan_size),
    };
    use_memo(|(store, deps)| func(store, deps), (store, deps))
}

#[yew::hook]
pub fn use_app_store_selector<Func, Resp: 'static>(func: Func) -> Rc<Resp>
where
    Func: Fn(&AppStore) -> Resp,
{
    use_app_store_selector_with_deps(|state, _deps| func(state), ())
}

pub fn select_fan_series_by_id(
    state: &AppStore,
    maybe_id: &Option<String>,
) -> Option<FanSeries<Vec<FanSize<()>>>> {
    if let Some(id) = maybe_id {
        let fan_series: FanSeries<()> = state.fan_series.fan_serieses.get(id)?.clone();
        let fan_sizes: Vec<FanSize<()>> = state
            .fan_size
            .fan_sizes
            .clone()
            .into_values()
            .filter(|fs| fs.fan_series_id == *id)
            .collect();
        Some((fan_series, fan_sizes).into())
    } else {
        None
    }
}

pub fn select_fan_size_by_id(state: &AppStore, id: &String) -> Option<FanSize<FanSeries<()>>> {
    let fan_size = state.fan_size.fan_sizes.get(id)?.clone();

    let fan_series = state
        .fan_series
        .fan_serieses
        .get(&fan_size.fan_series_id)?
        .clone();

    Some((fan_size, fan_series).into())
}

pub fn select_all_fan_series(state: &AppStore) -> Vec<FanSeries<()>> {
    state
        .fan_series
        .fan_serieses
        .clone()
        .into_values()
        .collect()
}

pub fn select_a1_report(
    state: &AppStore,
    maybe_id: &Option<String>,
) -> Option<A1Standard2010Report<FanSize<FanSeries<()>>>> {
    if let Some(id) = maybe_id {
        let maybe_report = state.a1_report.reports.get(id);
        if let Some(report) = maybe_report {
            let maybe_size = select_fan_size_by_id(state, &report.fan_size_id);
            if let Some(size) = maybe_size {
                return Some((report.clone(), size).into());
            }
        }
    }
    None
}

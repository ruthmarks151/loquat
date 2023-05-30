use std::ops::Deref;
use std::rc::Rc;

use loquat_common::{
    api::fan_series::UpdateBody,
    models::{FanSeries, FanSize},
};
use web_sys::MouseEvent;
use yew::{
    hook, use_callback, use_effect_with_deps, use_state, Callback, UseStateHandle,
};
use yewdux::prelude::use_store;

use crate::api::store::Store as ApiStore;
use crate::api::store::{ApiRequestAction, GetParameters, Gettable};
use crate::store::{select_fan_series_by_id, use_app_store_selector_with_deps};

pub struct FanSeriesFormHookRes {
    pub maybe_fan_series: Rc<Option<FanSeries<Vec<FanSize<()>>>>>,
    pub on_valid_entry: Callback<UpdateBody>,
    pub on_submit_click: Callback<MouseEvent>,
}

#[hook]
pub fn use_fan_series_form_controller(maybe_fan_series_id: Option<String>) -> FanSeriesFormHookRes {
    let (_state, api_dispatch) = use_store::<ApiStore>();

    let last_valid_entry: UseStateHandle<Option<UpdateBody>> = use_state(|| None);

    let maybe_fan_series: Rc<Option<FanSeries<Vec<FanSize<()>>>>> =
        use_app_store_selector_with_deps(select_fan_series_by_id, maybe_fan_series_id.clone());

    let on_valid_entry = {
        let set_last_valid_entry = last_valid_entry.setter();
        use_callback(move |entry, _| set_last_valid_entry.set(Some(entry)), ())
    };
    let on_submit_click = {
        use_callback(
            |_evt: MouseEvent, (dispatch, parsed_update_body_ref)| {
                if let Some(update_body) = (*parsed_update_body_ref).as_ref() {
                    dispatch.apply(ApiRequestAction::Get(
                        GetParameters { ignore_cache: true },
                        Gettable::PutFanSeriesReport {
                            body: update_body.clone(),
                        },
                    ))
                }
            },
            (api_dispatch.clone(), last_valid_entry.deref().clone()),
        )
    };

    use_effect_with_deps(
        {
            // let api_dispatch = api_dispatch.clone();
            move |maybe_fan_series_id: &Option<String>| {
                if let Some(fan_series_id) = maybe_fan_series_id {
                    api_dispatch.apply(ApiRequestAction::Get(
                        GetParameters {
                            ignore_cache: false,
                        },
                        Gettable::FanSeries {
                            id: fan_series_id.clone(),
                        },
                    ));
                }
                || {}
            }
        },
        maybe_fan_series_id,
    );

    FanSeriesFormHookRes {
        on_valid_entry,
        maybe_fan_series,
        on_submit_click,
    }
}

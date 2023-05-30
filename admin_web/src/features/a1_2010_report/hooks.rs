use std::ops::Deref;
use std::rc::Rc;

use loquat_common::{
    api::a1_2010_report::UpdateBody,
    models::{A1Standard2010Determination, A1Standard2010Report, FanSeries, FanSize},
};
use web_sys::MouseEvent;
use yew::{
    hook, use_callback, use_effect_with_deps, use_memo, use_state, Callback, UseStateHandle,
};
use yewdux::prelude::use_store;

use crate::api::store::Store as ApiStore;
use crate::api::store::{ApiRequestAction, GetParameters, Gettable};
use crate::store::{select_a1_report, use_app_store_selector_with_deps};

pub struct A1FormHookRes {
    pub on_valid_entry: Callback<UpdateBody>,
    pub maybe_report: Rc<Option<A1Standard2010Report<FanSize<FanSeries<()>>>>>,
    pub maybe_points_to_render: Rc<Option<Vec<A1Standard2010Determination>>>,
    pub on_submit_click: Callback<MouseEvent>,
}

#[hook]
pub fn use_a1_form_controller(maybe_report_id: Option<String>) -> A1FormHookRes {
    let (_state, api_dispatch) = use_store::<ApiStore>();

    let last_valid_entry: UseStateHandle<Option<UpdateBody>> = use_state(|| None);

    let maybe_report: Rc<Option<A1Standard2010Report<FanSize<FanSeries<()>>>>> =
        use_app_store_selector_with_deps(select_a1_report, maybe_report_id.clone());

    let on_valid_entry = {
        let set_last_valid_entry = last_valid_entry.setter();
        use_callback(move |entry, _| set_last_valid_entry.set(Some(entry)), ())
    };

    let maybe_points_to_render = use_memo(
        |parsed_update_body| {
            parsed_update_body
                .as_ref()
                .map(|u| Some(u.clone().determinations))
                .unwrap_or(maybe_report.as_ref().clone().map(|r| r.determinations))
        },
        last_valid_entry.deref().clone(),
    );

    let on_submit_click = {
        use_callback(
            |_evt: MouseEvent, (dispatch, parsed_update_body_ref)| {
                if let Some(update_body) = (*parsed_update_body_ref).as_ref() {
                    dispatch.apply(ApiRequestAction::Get(
                        GetParameters { ignore_cache: true },
                        Gettable::PutA12010Report {
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
            move |maybe_report_id: &Option<String>| {
                if let Some(report_id) = maybe_report_id {
                    api_dispatch.apply(ApiRequestAction::Get(
                        GetParameters {
                            ignore_cache: false,
                        },
                        Gettable::A1Report {
                            id: report_id.clone(),
                        },
                    ));
                }
                || {}
            }
        },
        maybe_report_id,
    );

    A1FormHookRes {
        on_valid_entry,
        maybe_report,
        maybe_points_to_render,
        on_submit_click,
    }
}

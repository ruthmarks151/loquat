use std::rc::Rc;

use loquat_common::models::FanSeries;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_callback, use_effect_with_deps, use_node_ref, Callback, Html,
    Properties,
};
use yewdux::prelude::{use_selector_with_deps, use_store, use_selector};

use crate::api::store::{RequestStatuses, Store as ApiStore};
use crate::features::fan_series::Store;
use crate::{
    api::store::{ApiRequestAction, GetParameters, Gettable},
    store::{select_all_fan_series, use_app_store_selector},
};

#[derive(Properties, PartialEq)]
pub struct FanSeriesPickerProps {
    pub no_selection_label: String,
    pub selection: Option<FanSeries<()>>,
    pub on_select: Callback<Option<FanSeries<()>>, ()>,
}

#[function_component]
pub fn FanSeriesPicker(
    FanSeriesPickerProps {
        no_selection_label,
        selection,
        on_select,
    }: &FanSeriesPickerProps,
) -> Html {
    let gettable = Gettable::FanSeriesesIndex;
    let select_ref = use_node_ref();

    let (_state, dispatch) = use_store::<ApiStore>();
    let fan_serieses: Rc<Vec<FanSeries<()>>> = use_selector(|state: &Store| state.fan_serieses.values().cloned().collect::<Vec<_>>());
    let request_status = use_selector_with_deps(
        |store: &ApiStore, dep_gettable| {
            store
                .get_status
                .get(dep_gettable)
                .cloned()
                .unwrap_or_default()
        },
        gettable.clone(),
    );

    use_effect_with_deps(
        move |dep_gettable| {
            dispatch.apply(ApiRequestAction::Get(
                GetParameters {
                    ignore_cache: false,
                },
                dep_gettable.clone(),
            ));

            || {}
        },
        gettable,
    );

    let selected_option: Html = match selection {
        Some(s) => html! {
          <option selected={true} value={s.id.clone()}>
            {s.id.clone()}{" "}{s.fan_type.to_string()}
          </option>
        },
        None => html! {
          <option selected={true} value={""}>
            {no_selection_label}
          </option>
        },
    };

    let all_options: Html = fan_serieses
        .iter()
        .map(|s| {
            html!(
              <option value={s.id.clone()}>
                {s.id.clone()}{" "}{s.fan_type.to_string()}
              </option>
            )
        })
        .collect::<Html>();

    let select_callback = use_callback(
        move |_evt: web_sys::Event, (on_select_dep, select_ref_dep, fan_serieses_ref)| {
            let select_el = select_ref_dep
                .cast::<HtmlInputElement>()
                .expect("select_ref not attached to select element");
            let selected_id = select_el.value();

            on_select_dep.emit(
                fan_serieses_ref
                    .iter()
                    .find(|fs| fs.id == selected_id)
                    .cloned(),
            );
        },
        (on_select.clone(), select_ref.clone(), fan_serieses),
    );

    let options: Html = match request_status.as_ref() {
        RequestStatuses::Error(_error_at, msg) => {
            html! {
              <>
                {selected_option}
                <option>{"Error loading fan series..."}</option>
              </>
            }
        }
        RequestStatuses::Unfetched => {
            html! {<>
                {selected_option}
                <option>{"Loading..."}</option>
                </>
            }
        }
        RequestStatuses::Fetching(_fetched_at) => {
            html! {
              <>
                {selected_option}
                <option>{"Loading..."}</option>
              </>
            }
        }
        RequestStatuses::Refetching(_fetched_at, _last_status) => {
            html! {
              <>
                {selected_option}
                <option>{"Loading..."}</option>
                {all_options}
              </>
            }
        }

        RequestStatuses::Fetched(fetched_at) => {
            html! {
              <>
                {selected_option}
                {all_options}
              </>
            }
        }
    };

    html! {
      <select onchange={select_callback} ref={select_ref}>
        {options}
      </select>
    }
}

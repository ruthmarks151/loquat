use std::rc::Rc;

use loquat_common::models::FanSize;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_callback, use_effect_with_deps, use_node_ref, Callback, Html,
    Properties,
};
use yewdux::prelude::{use_selector, use_selector_with_deps, use_store};

use crate::api::store::{RequestStatuses, Store as ApiStore};
use crate::features::fan_size::Store;
use crate::{
    api::store::{ApiRequestAction, GetParameters, Gettable},
    store::{select_all_fan_series, use_app_store_selector},
};

#[derive(Properties, PartialEq)]
pub struct FanSizePickerProps {
    pub fan_series_id: String,
    pub no_selection_label: String,
    pub selection: Option<FanSize<()>>,
    pub on_select: Callback<Option<FanSize<()>>, ()>,
}

#[function_component]
pub fn FanSizePicker(
    FanSizePickerProps {
        fan_series_id,
        no_selection_label,
        selection,
        on_select,
    }: &FanSizePickerProps,
) -> Html {
    let gettable = Gettable::FanSizesIndex;
    let select_ref = use_node_ref();

    let (_state, dispatch) = use_store::<ApiStore>();
    let fan_sizes: Rc<Vec<FanSize<()>>> = use_selector_with_deps(
        move |state: &Store, deps| {
            state
                .fan_sizes
                .values()
                .filter(|size| size.fan_series_id.clone() == (deps).clone())
                .cloned()
                .collect::<Vec<_>>()
        },
        fan_series_id.clone(),
    );
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
            {s.id.clone()}{" "}
          </option>
        },
        None => html! {
          <option selected={true} value={""}>
            {no_selection_label}
          </option>
        },
    };
    let selection_id = selection.as_ref().map_or("".to_string(), |s| s.id.clone());
    let all_options: Html = fan_sizes
        .iter()
        .filter(|opt| opt.id != selection_id)
        .map(|s| {
            html!(
              <option value={s.id.clone()}>
                {s.id.clone()}
              </option>
            )
        })
        .collect::<Html>();

    let select_callback = use_callback(
        move |_evt: web_sys::Event, (on_select_dep, select_ref_dep, fan_sizes_ref)| {
            let select_el = select_ref_dep
                .cast::<HtmlInputElement>()
                .expect("select_ref not attached to select element");
            let selected_id = select_el.value();

            on_select_dep.emit(
                fan_sizes_ref
                    .iter()
                    .find(|fs| fs.id == selected_id)
                    .cloned(),
            );
        },
        (on_select.clone(), select_ref.clone(), fan_sizes),
    );

    let options: Html = match request_status.as_ref() {
        RequestStatuses::Error(_error_at, msg) => {
            html! {
              <>
                {selected_option}
                <option>{"Error loading fan sizes..."}</option>
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

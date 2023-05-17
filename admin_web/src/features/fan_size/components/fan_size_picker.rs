use std::rc::Rc;

use loquat_common::models::FanSize;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_callback, use_effect_with_deps, use_node_ref, Callback, Html,
    Properties,
};
use yewdux::prelude::{use_selector, use_selector_with_deps, use_store};

use crate::api::store::{RequestStatuses, Store as ApiStore};
use crate::common::components::{Select, SelectOption};
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
    html! {
      <Select<FanSize<()>>
        no_selection_label={no_selection_label.clone()}
        selection={selection.clone()}
        on_select={on_select}
        request_status={request_status}
        selectables={fan_sizes}
      />
    }
}

impl SelectOption for FanSize<()> {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn label(&self) -> String {
        self.id.clone()
    }
}

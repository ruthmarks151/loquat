use std::rc::Rc;

use loquat_common::models::FanSeries;
use yew::{function_component, html, use_effect_with_deps, Callback, Html, Properties};
use yewdux::prelude::{use_selector, use_selector_with_deps, use_store};

use crate::api::store::Store as ApiStore;
use crate::api::store::{ApiRequestAction, GetParameters, Gettable};
use crate::common::components::{select::SelectOption, Select};
use crate::features::fan_series::Store;

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
    let (_state, dispatch) = use_store::<ApiStore>();
    let fan_serieses: Rc<Vec<FanSeries<()>>> =
        use_selector(|state: &Store| state.fan_serieses.values().cloned().collect::<Vec<_>>());
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
      <Select<FanSeries<()>>
        no_selection_label={no_selection_label.clone()}
        selection={selection.clone()}
        on_select={on_select}
        request_status={request_status}
        selectables={fan_serieses}
      />
    }
}

impl SelectOption for FanSeries<()> {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn label(&self) -> String {
        format!("{} {}", self.id.clone(), self.fan_type.to_string())
    }
}

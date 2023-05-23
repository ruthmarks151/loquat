use std::ops::Deref;

use loquat_common::models::{FanSeries, FanSize};
use yew::{
    function_component, html, use_callback, use_effect_with_deps, Html, Properties, UseStateHandle,
};

use crate::features::{fan_series::FanSeriesPicker, fan_size::FanSizePicker};

#[derive(Properties, PartialEq)]
pub struct FanSeriesAndSizePickerProps {
    #[prop_or(vec![])]
    pub size_errs: Vec<String>,
    pub saved_size: Option<FanSize<()>>,
    pub picked_fan_series_state: UseStateHandle<Option<FanSeries<()>>>,
    pub picked_fan_size_state: UseStateHandle<Option<FanSize<()>>>,
}

#[function_component]
pub fn FanSeriesAndSizePicker(
    FanSeriesAndSizePickerProps {
        size_errs,
        saved_size,
        picked_fan_series_state,
        picked_fan_size_state,
    }: &FanSeriesAndSizePickerProps,
) -> Html {
    // On Fan series change, handele updating the state dropdown
    use_effect_with_deps(
        {
            let picked_fan_size_state = picked_fan_size_state.clone();
            let saved_size = saved_size.clone();
            move |fan_series_id: &Option<String>| {
                if let Some(selected_fan_series_id) = fan_series_id {
                    if saved_size
                        .as_ref()
                        .map_or(false, |es| *selected_fan_series_id == es.fan_series_id)
                    {
                        picked_fan_size_state.set(saved_size);
                    } else if let Some(picked_fan_size) = (*picked_fan_size_state).clone() {
                        if picked_fan_size.fan_series_id != *selected_fan_series_id {
                            picked_fan_size_state.set(None);
                        }
                    }
                }
            }
        },
        picked_fan_series_state
            .as_ref()
            .map(|fs| fs.id.clone()),
    );

    let series_picker = {
        let set_picked_fan_series = picked_fan_series_state.setter();
        html! {
            <FanSeriesPicker
                selection={picked_fan_series_state.deref().clone()}
                no_selection_label={"--"}
                on_select={
                   move |value| set_picked_fan_series.set(value)
            }
            />
        }
    };

    let size_picker = {
        let picked_fan_size_setter = picked_fan_size_state.setter();
        let fan_series_id = picked_fan_series_state.deref().clone().map(|fs| fs.id);
        let option_predicate = use_callback(
            |fs: FanSize<()>, fan_series_id| match fan_series_id {
                Some(fan_series_id) => &fs.fan_series_id == fan_series_id,
                None => true,
            },
            fan_series_id,
        );
        html! {
            <FanSizePicker
                errs={size_errs.clone()}
                option_predicate={option_predicate}
                selection={picked_fan_size_state.deref().clone()}
                no_selection_label={"--"}
                on_select={move |s| picked_fan_size_setter.set(s)}
            />
        }
    };

    html! {
      <>
        <label>{"Fan Series"}</label>
        {series_picker}
        <label>{"Fan Size"}</label>
        {size_picker}
      </>
    }
}

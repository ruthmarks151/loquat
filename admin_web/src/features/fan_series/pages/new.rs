use yew::{function_component, html, Html};

use crate::features::fan_series::{
    components::FanSeriesForm,
    hooks::{use_fan_series_form_controller, FanSeriesFormHookRes},
};

#[function_component]
pub fn NewFanSeriesPage() -> Html {
    let FanSeriesFormHookRes {
        maybe_fan_series,
        on_valid_entry,
        on_submit_click,
    } = use_fan_series_form_controller(None);

    html! {
        <>
            <h1>{"New Fan Series"}</h1>
            <div style="display: flex; flex-direction: row;">
                <FanSeriesForm
                    {maybe_fan_series}
                    {on_valid_entry}
                    {on_submit_click}
                />
            </div>
        </>
    }
}

use yew::{function_component, html, Html, Properties};
use yew_router::prelude::Link;

use crate::{features::fan_series::{
    components::FanSeriesForm,
    hooks::{use_fan_series_form_controller, FanSeriesFormHookRes},
}, route::Route};

#[derive(Properties, PartialEq)]
pub struct EditFanSeriesPageProps {
    pub id: String
}

#[function_component]
pub fn EditFanSeriesPage(EditFanSeriesPageProps { id }: &EditFanSeriesPageProps) -> Html {
    let fan_series_id = id.replace("%20", " ");

    let FanSeriesFormHookRes {
        maybe_fan_series,
        on_valid_entry,
        on_submit_click,
    } = use_fan_series_form_controller(Some(fan_series_id.clone()));

    html! {
        <>
            <h1>
                <Link<Route> to={Route::GetFanSeries { id: id.clone() }}>
                    {'\u{2b05}'} // Fat Left Arrow
                    {'\u{2002}'} // en-space
                </Link<Route>>
                {format!("Fan Series {}", fan_series_id)}
            </h1>
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

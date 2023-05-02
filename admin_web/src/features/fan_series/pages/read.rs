use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::features::fan_series::api::get_fan_series;

#[derive(Properties, PartialEq)]
pub struct ReadFanSeriesPageProps {
    pub id: String,
}

#[function_component]
pub fn ReadFanSeriesPage(props: &ReadFanSeriesPageProps) -> Html {
    let data = use_state(|| None);

    {
        let data = data.clone();
        let id = props.id.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    data.set(Some(get_fan_series(id).await));
                });
            }
            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div>
                    <h1>{"Fan Size Detail"}</h1>
                    {"id: "} {data.id.to_owned()} <br/>
                    {"fan_type: "} {data.fan_type.to_string()}
                </div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }
}

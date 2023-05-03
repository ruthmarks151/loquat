use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::route::Route;

use super::super::api::get_fan_series;

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
                    {"fan_type: "} {data.fan_series.fan_type.to_string()} <br/>
                    {"id: "} {data.fan_series.id.to_owned()}
                    <h2>{"Sizes"}</h2>
                    <ul>
                        { data.fan_sizes.iter().map(|fan_size| html! {
                            <li>
                                <Link<Route> to={Route::GetFanSize { id: fan_size.id.clone() }}>
                                    {fan_size.id.clone()}{" Diameter: "}{fan_size.diameter.clone()}
                                </Link<Route>>
                            </li>
                          } ).collect::<Vec<_>>() }
                    </ul>
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

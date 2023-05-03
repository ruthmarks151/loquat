use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect, use_state, Html};
use yew_router::prelude::Link;

use super::super::api::index_fan_serieses;
use crate::route::Route;

#[function_component]
pub fn IndexFanSeriesPage() -> Html {
    let data = use_state(|| None);
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    data.set(Some(index_fan_serieses().await));
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
                    <h1>{"Fan List"}</h1>
                    <ul>
                        { data.iter().map(|fan| html! {
                            <li>
                                <Link<Route> to={Route::GetFanSeries { id: fan.id.clone() }}>
                                    {fan.id.clone()}
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

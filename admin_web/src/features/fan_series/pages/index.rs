use crate::api::store::{RequestStatuses, Store as ApiStore};
use instant::Instant;

use loquat_common::models::FanSeries;
use yew::{function_component, html, use_effect_with_deps, Html};
use yew_router::prelude::Link;
use yewdux::prelude::{use_selector, use_selector_with_deps, use_store};

use crate::{
    api::store::{ApiRequestAction, GetParameters, Gettable},
    route::Route,
    store::{select_all_fan_series, use_app_store_selector},
};

#[function_component]
pub fn IndexFanSeriesPage() -> Html {
    let gettable = Gettable::FanSeriesesIndex;
    let (_state, dispatch) = use_store::<ApiStore>();
    let fan_serieses: Vec<FanSeries<()>> = use_app_store_selector(select_all_fan_series);
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

    let reload_callback = {
        let gettable_dup = gettable.clone();
        dispatch.apply_callback(move |_evt| {
            ApiRequestAction::Get(GetParameters { ignore_cache: true }, gettable_dup.clone())
        })
    };

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
    match request_status.as_ref() {
        RequestStatuses::Error(_error_at, msg) => {
            html! {
                <div>
                    <h1>{"Fan List"} <button onclick={reload_callback}>{"Refresh"}</button></h1>

                    {"Error: "}{msg}
                </div>
            }
        }
        RequestStatuses::Unfetched => {
            html! {
                <div>
                    <h1>{"Fan List"}</h1>

                    {"Click to fetch"}
                </div>
            }
        }
        RequestStatuses::Fetching(_fetched_at) => {
            html! {
                <div>
                    <h1>{"Fan List"}</h1>

                    {"Loading..."}
                </div>
            }
        }
        RequestStatuses::Refetching(_fetched_at, _last_status) => {
            html! {
                <div>
                    <h1>{"Fan List"}</h1>

                    {"Loading..."}
                </div>
            }
        }

        RequestStatuses::Fetched(fetched_at) => {
            if fan_serieses.is_empty() {
                html! {
                    <div>
                    <h1>{"Fan List"} {format!("Fetched {:#?} ago", (Instant::now() - (*fetched_at )))}</h1>

                    {"No Serieses!"}
                    </div>
                }
            } else {
                html! {
                    <div>
                        <h1>{"Fan List"} {format!("Fetched {:#?} ago", (Instant::now() - (*fetched_at )))}</h1>
                        <ul>
                            { fan_serieses.iter().map(|fan| html! {
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
        }
    }
}

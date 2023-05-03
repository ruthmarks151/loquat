use std::rc::Rc;

use loquat_common::models::{fan_series::FanSeries, fan_size::FanSize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::Link;
use yewdux::prelude::{use_selector_with_deps, use_store};

use crate::{
    route::Route,
    store::{
        selectors::{select_fan_series_by_id, select_fan_sizes_for_fan_series_id},
        FanStore, FanStoreActions,
    },
};

use super::super::api::get_fan_series;

#[derive(Properties, PartialEq)]
pub struct ReadFanSeriesPageProps {
    pub id: String,
}

#[function_component]
pub fn ReadFanSeriesPage(props: &ReadFanSeriesPageProps) -> Html {
    let (_state, dispatch) = use_store::<FanStore>();
    let id = props.id.clone();

    let format_id = id.replace("%20", " ");
    let fan_series_option: Rc<Option<FanSeries>> =
        use_selector_with_deps(select_fan_series_by_id, format_id);

    let fan_series_id = id.replace("%20", " ");
    let fan_sizes: Rc<Vec<FanSize>> =
        use_selector_with_deps(select_fan_sizes_for_fan_series_id, fan_series_id);

    {
        use_effect(move || {
            spawn_local(async move {
                let response = get_fan_series(id).await;
                if let Ok(response_json) = response {
                    dispatch.apply(FanStoreActions::InsertFanSeries(response_json.fan_series));

                    for fan_size in response_json.fan_sizes {
                        dispatch.apply(FanStoreActions::InsertFanSize(fan_size));
                    }
                };
            });
            || {}
        });
    }

    match fan_series_option.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(data) => {
            html! {
                <div>
                    <h1>{"Fan Size Detail"}</h1>
                    {"fan_type: "} {data.fan_type.to_string()} <br/>
                    {"id: "} {data.id.to_owned()}
                    <h2>{"Sizes"}</h2>
                    <ul>
                        { fan_sizes.iter().map(|fan_size| html! {
                            <li>
                                <Link<Route> to={Route::GetFanSize { id: fan_size.id.clone() }}>
                                    {fan_size.id.clone()}{" Diameter: "}{fan_size.diameter}
                                </Link<Route>>
                            </li>
                          } ).collect::<Vec<_>>() }
                    </ul>
                </div>
            }
        }
    }
}

use crate::api::store::Store as ApiStore;
use crate::{
    api::store::{ApiRequestAction, GetParameters, Gettable},
    route::Route,
    store::{select_fan_series_by_id, use_app_store_selector_with_deps},
};
use loquat_common::models::{FanSeries, FanSize};
use yew::prelude::*;
use yew_router::prelude::Link;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct ReadFanSeriesPageProps {
    pub id: AttrValue,
}

#[function_component]
pub fn ReadFanSeriesPage(props: &ReadFanSeriesPageProps) -> Html {
    let (_state, dispatch) = use_store::<ApiStore>();
    let id = props.id.clone();

    let format_id = id.replace("%20", " ");
    let fan_series_option: Option<FanSeries<Vec<FanSize<()>>>> =
        use_app_store_selector_with_deps(select_fan_series_by_id, format_id.clone());

    use_effect_with_deps(
        move |_| {
            dispatch.apply(ApiRequestAction::Get(
                GetParameters {
                    ignore_cache: false,
                },
                Gettable::FanSeries { id: format_id },
            ));
            || {}
        },
        (),
    );

    match fan_series_option {
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
                        { data.fan_sizes.iter().map(|fan_size| html! {
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

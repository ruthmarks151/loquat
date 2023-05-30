use std::rc::Rc;

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
    let fan_series_option: Rc<Option<FanSeries<Vec<FanSize<()>>>>> =
        use_app_store_selector_with_deps(select_fan_series_by_id, Some(format_id.clone()));

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

    match fan_series_option.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(data) => {
            html! {
                <div>
                    <h1>
                        <Link<Route> to={Route::IndexFanSerieses}>
                            {'\u{2b05}'} // Fat arrow
                            {'\u{2002}'} // en-space
                        </Link<Route>>
                        {"Fan Series Detail"}
                        <Link<Route> to={Route::EditFanSeries { id: id.to_string() }}>
                            {'\u{2002}'} // En-space
                            {"Edit"}
                        </Link<Route>>
                    </h1>
                    <table>
                        <tr>
                            <td>
                                {"id: "}
                            </td>
                            <td>
                                {data.id.to_owned()}
                            </td>
                        </tr>
                        <tr>
                            <td>
                                {"Fan Type: "}
                            </td>
                            <td>
                                {data.fan_type.to_string()}
                            </td>
                        </tr>
                        <tr>
                            <td>
                                {"Sizes"}
                            </td>
                            <td>
                            <ul style="margin: 0; padding-inline-start: 1.25em;">
                                { data.fan_sizes.iter().map(|fan_size| html! {
                                    <li>
                                        <Link<Route> to={Route::GetFanSize { id: fan_size.id.clone() }}>
                                            {fan_size.id.clone()}{" Diameter: "}{fan_size.diameter}
                                        </Link<Route>>
                                    </li>
                                } ).collect::<Html>() }
                        </ul>
                            </td>
                        </tr>
                    </table>                    
                </div>
            }
        }
    }
}

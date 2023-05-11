use std::rc::Rc;

use loquat_common::models::FanSeries;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect, Html};
use yew_router::prelude::Link;
use yewdux::prelude::{use_selector, use_store};

use crate::{
    route::Route,
    store::{selectors::select_all_fan_series, FanStore, FanStoreActions},
};

#[function_component]
pub fn IndexFanSeriesPage() -> Html {
    let (_state, dispatch) = use_store::<FanStore>();
    let fan_serieses: Rc<Vec<FanSeries<()>>> = use_selector(select_all_fan_series);
    use_effect( move || { 
        dispatch.apply(FanStoreActions::IndexFanSeries);
        return || {} 
    });

    if fan_serieses.is_empty() {
        return html! {
            <div>{"No Serieses!"}</div>
        };
    }

    html! {
        <div>
            <h1>{"Fan List"}</h1>
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

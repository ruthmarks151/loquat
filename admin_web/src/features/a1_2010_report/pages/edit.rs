use crate::api::store::Store as ApiStore;
use crate::features::a1_2010_report::Store;
use crate::features::fan_series::FanSeriesPicker;
use crate::store::select_a1_report;
use crate::{
    api::store::{ApiRequestAction, GetParameters, Gettable},
    route::Route,
    store::{select_fan_series_by_id, use_app_store_selector, use_app_store_selector_with_deps},
};
use loquat_common::models::{A1Standard2010Report, FanSeries, FanSize};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct EditA1PageProps {
    pub id: Option<String>,
}

#[function_component]
pub fn EditA1Page(props: &EditA1PageProps) -> Html {
    let (_state, _dispatch) = use_store::<Store>();

    let (_state, dispatch) = use_store::<ApiStore>();
    let id = props.id.clone();

    let format_id = id.map(|id| id.replace("%20", " "));
    let report_option: Option<A1Standard2010Report<FanSize<FanSeries<()>>>> =
        use_app_store_selector_with_deps(select_a1_report, format_id.clone());

    let fan_series_option = use_state(|| None);

    use_effect_with_deps(
        move |(fan_series_option_dep, new_fan_series)| {
            fan_series_option_dep.set(new_fan_series.clone())
        },
        (fan_series_option.clone(), report_option.clone().map(|r| r.fan_size.fan_series))
    );

    use_effect_with_deps(
        move |_| {
            if let Some(id) = format_id {
                dispatch.apply(ApiRequestAction::Get(
                    GetParameters {
                        ignore_cache: false,
                    },
                    Gettable::A1Report { id },
                ));
            }
            || {}
        },
        (),
    );
    let series_picker = html! {<FanSeriesPicker selection={(*fan_series_option).clone()} no_selection_label={"--"} on_select={move |s| fan_series_option.set(s)} />};
    
    match report_option {
        None => {
            html! {
                <div>
                    <h1>{"New A1 Report"}</h1>
                    {series_picker}
                </div>

            }
        }
        Some(report) => {
            html! { <div>
                <h1>{"Edit A1"}{ report.id.to_owned() }</h1>
                {series_picker}
                </div>
            }
        }
    }
}

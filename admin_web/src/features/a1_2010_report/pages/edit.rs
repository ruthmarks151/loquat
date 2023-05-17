use std::borrow::Borrow;
use std::iter;
use std::ops::DerefMut;

use crate::api::store::Store as ApiStore;
use crate::features::a1_2010_report::components::DeterminationTable;
use crate::features::a1_2010_report::components::_DeterminationTableProps::headers;
use crate::features::a1_2010_report::Store;
use crate::features::fan_series::{self, FanSeriesPicker};
use crate::features::fan_size::{self, FanSizePicker};
use crate::store::select_a1_report;
use crate::{
    api::store::{ApiRequestAction, GetParameters, Gettable},
    route::Route,
    store::{select_fan_series_by_id, use_app_store_selector, use_app_store_selector_with_deps},
};
use loquat_common::models::{
    A1Standard2010Determination, A1Standard2010Report, FanSeries, FanSize,
};
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

    let determinations_table_data: UseStateHandle<Vec<[Option<f64>; 3]>> =
        use_state(|| vec![[None, None, None]]);

    use_effect_with_deps(
        {
            let determinations_table_data_state = determinations_table_data.clone().clone();
            move |determinations_option: &Option<Vec<A1Standard2010Determination>>| {
                let rows: Vec<[Option<f64>; 3]> = match determinations_option {
                    Some(determinations) => determinations
                        .iter()
                        .map(|d| {
                            [
                                Some(d.static_pressure),
                                Some(d.cfm),
                                Some(d.brake_horsepower),
                            ]
                        })
                        .chain(iter::repeat([None, None, None]))
                        .take(10)
                        .collect(),
                    None => vec![[None, None, None]],
                };
                
                determinations_table_data_state.set(rows);
            }
        },
        report_option.clone().map(|r| r.determinations),
    );

    let fan_series_option = use_state(|| None);
    let fan_size_option = use_state(|| None);
    use_effect_with_deps(
        {
            let fan_series_option = fan_series_option.clone();
            let fan_size_option = fan_size_option.clone();
            move |new_fan_size_option: &Option<FanSize<FanSeries<()>>>| {
                if let Some(new_fan_size) = new_fan_size_option {
                    let (new_fan_size, new_fan_series): (FanSize<()>, FanSeries<()>) =
                        new_fan_size.clone().into();
                    fan_series_option.set(Some(new_fan_series));
                    fan_size_option.set(Some(new_fan_size));
                }
            }
        },
        report_option
            .clone()
            .map(|r: A1Standard2010Report<FanSize<FanSeries<()>>>| r.fan_size),
    );

    use_effect_with_deps(
        {
            let fan_size_option = fan_size_option.clone();
            let existing_size = report_option.clone().map(|r| {
                let (fan_size, _fan_series) = r.fan_size.into();
                fan_size
            });
            move |fan_series_id: &Option<String>| {
                if let Some(selected_fan_series_id) = fan_series_id {
                    if existing_size
                        .as_ref()
                        .map_or(false, |es| *selected_fan_series_id == es.fan_series_id)
                    {
                        fan_size_option.set(existing_size);
                    } else if let Some(picked_fan_size) = (*fan_size_option).clone() {
                        if picked_fan_size.fan_series_id != *selected_fan_series_id {
                            fan_size_option.set(None);
                        }
                    }
                }
            }
        },
        fan_series_option.as_ref().map(|fs| fs.id.clone()).into(),
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

    let on_determination_value_change: Callback<(usize, usize, Option<f64>)> = use_callback(
        move |(row_index, col_index, value), determinations_table_data_state_ref| {
            log::info!("Change in dep {:?}", (row_index, col_index, value));
            let mut new_rows: Vec<[Option<f64>; 3]> =
                (**determinations_table_data_state_ref).clone();
            let row_ref: &mut [Option<f64>; 3] =
                new_rows.get_mut(row_index).expect("Expected a row");
            row_ref[col_index] = value;
            determinations_table_data_state_ref.set(new_rows);
        },
        determinations_table_data.clone(),
    );

    

    let picked_series_id: String = (*fan_series_option)
        .clone()
        .map_or("".to_string(), |fs| fs.id);
    let series_picker = html! {
        <FanSeriesPicker
            selection={(*fan_series_option).clone()}
            no_selection_label={"--"}
            on_select={move |s| fan_series_option.set(s)}
        />
    };

    let size_picker = html! {
    <FanSizePicker
        fan_series_id={picked_series_id}
        selection={(*fan_size_option).clone()}
        no_selection_label={"--"}
        on_select={move |s| fan_size_option.set(s)}
    /> };

    match report_option {
        None => {
            html! {
                <div>
                    <h1>{"New A1 Report"}</h1>
                    {series_picker}
                    {size_picker}
                </div>

            }
        }
        Some(report) => {
            let headers_lables: [String; 3] = [
                "Static Pressure (in. wg)".to_string(),
                "Flow Rate (cfm)".to_string(),
                "Brake Horsepower (hp)".to_string(),
            ];
            html! { 
                <div>
                    <h1>{"Edit A1"}{ report.id.to_owned() }</h1>
                    {series_picker}
                    {size_picker}
                    <DeterminationTable<3> 
                        onchange={on_determination_value_change} 
                        headers={headers_lables} 
                        rows={(*determinations_table_data).clone()} 
                    />
                </div>
            }
        }
    }
}

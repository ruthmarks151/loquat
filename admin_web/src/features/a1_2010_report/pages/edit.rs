use std::iter;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::use_store;

use loquat_common::models::{
    A1Standard2010Determination, A1Standard2010Report, FanSeries, FanSize,
};

use crate::api::store::Store as ApiStore;
use crate::features::a1_2010_report::components::{DeterminationTable, FloatInput};
use crate::features::a1_2010_report::Store;
use crate::features::fan_series::{self, FanSeriesPicker};
use crate::features::fan_size::{self, FanSizePicker};
use crate::store::select_a1_report;
use crate::{
    api::store::{ApiRequestAction, GetParameters, Gettable},
    store::use_app_store_selector_with_deps,
};



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

    let rpm_inp: UseStateHandle<Option<f64>> = use_state(|| None);

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

    use_effect_with_deps(
        {
            let rpm_inp_state = rpm_inp.clone().clone();
            move |rpm_option: &Option<f64>| {
                rpm_inp_state.set(rpm_option.clone());
            }
        },
        report_option.clone().map(|r| r.parameters.rpm),
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

    let on_rpm_input_change = use_callback(
        move |(_index, rpm_option), rpm_inp_state_ref| rpm_inp_state_ref.set(rpm_option),
        rpm_inp.clone(),
    );

    let picked_series_id: String = (*fan_series_option)
        .clone()
        .map_or("".to_string(), |fs| fs.id);

    let determination_paste_ref = use_node_ref();
    let on_determination_paste = {
        let determination_paste_ref = determination_paste_ref.clone();
        let determinations_table_data = determinations_table_data.clone();
        use_callback(
            move |_evt, _dep| {
                let input = determination_paste_ref
                    .cast::<HtmlInputElement>()
                    .expect("input_ref not attached to input element");

                let input_val: String = input.value();
                let mut text_rows = input_val.split("\n");
                if text_rows.next() != Some("Det. No. P t P v P s Q H K p η t η s") {
                    log::warn!("First header doesn't match")
                }
                if text_rows.next() != Some("(in. wg) (in. wg) (in. wg) (cfm) (hp) - (%) (%)") {
                    log::warn!("Second header doesn't match")
                }
                let grid: Vec<[Option<f64>; 3]> = text_rows
                    .map(|row_str| {
                        let split_row = row_str.split_whitespace().collect::<Vec<_>>();
                        if split_row.len() != 9 {
                            log::warn!("Row length isn't right");
                            [None, None, None]
                        } else {
                            [
                                split_row.get(3).and_then(|s| s.parse::<f64>().ok()),
                                split_row.get(4).and_then(|s| s.parse::<f64>().ok()),
                                split_row.get(5).and_then(|s| s.parse::<f64>().ok()),
                            ]
                        }
                    })
                    .collect::<Vec<_>>()
                    .clone();
                determinations_table_data.set(grid);
            },
            (),
        )
    };

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
                    <h1>{"Test No. "}{ report.id.to_owned() }</h1>
                    <h2>{"Test Details"}</h2>
                    <div style="display: grid; grid-template-columns: auto auto; width: fit-content; column-gap: 8px; row-gap: 4px;">
                        <label>{"Tested Fan Series"}</label>
                        {series_picker}
                        <label>{"Tested Fan Size"}</label>
                        {size_picker}
                        <label>{"Test RPM"}</label>
                        <FloatInput
                            value={(*rpm_inp).clone()}
                            index={0}
                            onchange={on_rpm_input_change}
                        />
                    </div>
                    <label><h2>{"Determination Points"}</h2></label>
                    <DeterminationTable<3>
                        onchange={on_determination_value_change}
                        headers={headers_lables}
                        rows={(*determinations_table_data).clone()}
                    />
                    <label><h3>{"Quick Paste Determination Points"}</h3></label>
                    <textarea ref={determination_paste_ref} rows={"13"} cols={"50"} onblur={on_determination_paste} >
                    </textarea>
                </div>
            }
        }
    }
}

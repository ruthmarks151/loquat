use std::iter;

use loquat_common::api::a1_2010_report::UpdateBody;
use serde::__private::de;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::use_store;

use loquat_common::models::{
    A1Standard2010Determination, A1Standard2010Report, FanSeries, FanSize,
};

use crate::api::store::Store as ApiStore;
use crate::common::components::determination_table::FloatInput;
use crate::features::a1_2010_report::components::A12010DeterminationTable;
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

    let rpm_inp: UseStateHandle<String> = use_state(|| "".to_string());

    let determinations_table_data: UseStateHandle<Vec<[String; 3]>> = use_state(|| vec![]);

    use_effect_with_deps(
        {
            let determinations_table_data_state = determinations_table_data.clone().clone();
            move |determinations_option: &Option<Vec<A1Standard2010Determination>>| {
                log::info!("Effect updating data! ");
                determinations_table_data_state.set(
                    determinations_option
                        .clone()
                        .map(|det_vec| {
                            det_vec
                                .into_iter()
                                .map(|det| {
                                    [
                                        det.static_pressure.to_string(),
                                        det.cfm.to_string(),
                                        det.brake_horsepower.to_string(),
                                    ]
                                })
                                .collect()
                        })
                        .unwrap_or(vec![]),
                );
            }
        },
        report_option.clone().map(|r| r.determinations),
    );

    use_effect_with_deps(
        {
            let rpm_inp_state = rpm_inp.clone().clone();
            move |rpm_option: &Option<f64>| {
                rpm_inp_state.set(rpm_option.map_or("".to_string(), |r| r.to_string()));
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

    let handle_save_callback = {
        use_callback(
            |_evt: MouseEvent,
             (dispatch, id_opt, fan_rpm_opt, determinations_vec, fan_size_id_opt)| {
                let update_body: Option<UpdateBody> = (|| {
                    let fan_rpm = (*fan_rpm_opt).parse::<f64>().ok()?;
                    let fan_size_id = fan_size_id_opt.clone()?;
                    let parsed_determinations_vec = determinations_vec.into_iter().filter_map(|[static_pressure, cfm, brake_horsepower]| -> Option<A1Standard2010Determination> 
                    { Some(A1Standard2010Determination {
                    brake_horsepower: brake_horsepower.parse().ok()?, cfm: cfm.parse().ok()?, static_pressure: static_pressure.parse().ok()?
                })}).collect();
                    Some(UpdateBody {
                        id: id_opt.clone().unwrap().to_string(),
                        determinations: parsed_determinations_vec,
                        fan_rpm,
                        fan_size_id,
                    })
                })();

                dispatch.apply(ApiRequestAction::Get(
                    GetParameters { ignore_cache: true },
                    Gettable::PutA12010Report {
                        body: update_body.unwrap(),
                    },
                ))
            },
            (
                dispatch.clone(),
                report_option.clone().map(|r| r.id),
                (*rpm_inp).clone(),
                (*determinations_table_data).clone(),
                (*fan_size_option).clone().map(|fs| fs.id),
            ),
        )
    };

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

    let on_dets_input_change = {
        let determinations_table_data = determinations_table_data.clone();
        use_callback(
            move |data: [[String; 3]; 10], _dets| determinations_table_data.set(data.to_vec()),
            (),
        )
    };

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
                let grid: Vec<[String; 3]> = text_rows
                    .map(|row_str| {
                        let split_row = row_str.split_whitespace().collect::<Vec<_>>();
                        if split_row.len() != 9 {
                            log::warn!("Row length isn't right");
                            ["".to_string(), "".to_string(), "".to_string()]
                        } else {
                            [
                                split_row.get(3).map_or("".to_string(), |s| s.to_string()),
                                split_row.get(4).map_or("".to_string(), |s| s.to_string()),
                                split_row.get(5).map_or("".to_string(), |s| s.to_string()),
                            ]
                        }
                    })
                    .collect::<Vec<_>>()
                    .clone();
                if grid.len() != 12 {
                    log::warn!("Paste doesn't have the correct number of rows");
                    return;
                }
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
                    <A12010DeterminationTable 
                        fields={(*determinations_table_data).clone()} 
                        onchange={on_dets_input_change} 
                    />
                    <label><h3>{"Quick Paste Determination Points"}</h3></label>
                    <textarea ref={determination_paste_ref} rows={"13"} cols={"50"} onblur={on_determination_paste} >
                    </textarea>
                    <button onclick={handle_save_callback}>{"Save"}</button>
                </div>
            }
        }
    }
}

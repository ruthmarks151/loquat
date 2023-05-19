use std::ops::Deref;

use loquat_common::api::a1_2010_report::UpdateBody;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::use_store;

use loquat_common::models::{
    A1Standard2010Determination, A1Standard2010Report, FanSeries, FanSize,
};

use crate::api::store::Store as ApiStore;
use crate::common::components::determination_table::FloatInput;
use crate::features::a1_2010_report::components::A12010DeterminationTable;
use crate::features::fan_series::FanSeriesPicker;
use crate::features::fan_size::FanSizePicker;
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
    let (_state, api_dispatch) = use_store::<ApiStore>();

    let report_id = props.id.as_ref().map(|id| id.replace("%20", " "));
    let maybe_report: Option<A1Standard2010Report<FanSize<FanSeries<()>>>> =
        use_app_store_selector_with_deps(select_a1_report, report_id.clone());

    let picked_fan_series_state: UseStateHandle<Option<FanSeries<()>>> = use_state(|| None);
    let picked_fan_size_state: UseStateHandle<Option<FanSize<()>>> = use_state(|| None);
    let entered_rpm_state: UseStateHandle<String> = use_state(|| "".to_string());
    let determinations_state: UseStateHandle<Vec<[String; 3]>> = use_state(|| vec![]);

    // Reset fields for new report
    use_effect_with_deps(
        {
            let rpm_string_setter = entered_rpm_state.setter();
            let picked_fan_series_setter = picked_fan_series_state.setter();
            let picked_fan_size_setter = picked_fan_size_state.setter();
            let determinations_setter = determinations_state.setter();
            move |report_option: &Option<A1Standard2010Report<FanSize<FanSeries<()>>>>| {
                if let Some(report) = report_option {
                    let (new_fan_size, new_fan_series): (FanSize<()>, FanSeries<()>) =
                        report.fan_size.clone().into();
                    picked_fan_series_setter.set(Some(new_fan_series));
                    picked_fan_size_setter.set(Some(new_fan_size));
                    rpm_string_setter.set(report.parameters.rpm.to_string());
                    determinations_setter.set(
                        report
                            .determinations
                            .clone()
                            .into_iter()
                            .map(|det| {
                                [
                                    det.static_pressure.to_string(),
                                    det.cfm.to_string(),
                                    det.brake_horsepower.to_string(),
                                ]
                            })
                            .collect(),
                    );
                } else {
                    picked_fan_series_setter.set(None);
                    picked_fan_size_setter.set(None);
                    rpm_string_setter.set("".to_string());
                    determinations_setter.set(vec![])
                }
            }
        },
        maybe_report.clone(),
    );

    // On Fan series change, handele updating the state dropdown
    use_effect_with_deps(
        {
            let picked_fan_size_state = picked_fan_size_state.clone();
            let reports_maybe_size = maybe_report.clone().map(|r| {
                let (fan_size, _fan_series) = r.fan_size.into();
                fan_size
            });
            move |fan_series_id: &Option<String>| {
                if let Some(selected_fan_series_id) = fan_series_id {
                    if reports_maybe_size
                        .as_ref()
                        .map_or(false, |es| *selected_fan_series_id == es.fan_series_id)
                    {
                        picked_fan_size_state.set(reports_maybe_size);
                    } else if let Some(picked_fan_size) = (*picked_fan_size_state).clone() {
                        if picked_fan_size.fan_series_id != *selected_fan_series_id {
                            picked_fan_size_state.set(None);
                        }
                    }
                }
            }
        },
        picked_fan_series_state
            .as_ref()
            .map(|fs| fs.id.clone())
            .into(),
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
                api_dispatch.clone(),
                maybe_report.clone().map(|r| r.id),
                (*entered_rpm_state).clone(),
                (*determinations_state).clone(),
                (*picked_fan_size_state).clone().map(|fs| fs.id),
            ),
        )
    };

    use_effect_with_deps(
        move |_| {
            if let Some(id) = report_id {
                api_dispatch.apply(ApiRequestAction::Get(
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
        let determinations_table_setter = determinations_state.setter();
        use_callback(
            move |data: [[String; 3]; 10], _dets| determinations_table_setter.set(data.to_vec()),
            (),
        )
    };

    let on_rpm_input_change = {
        let entered_rpm_setter = entered_rpm_state.setter();
        use_callback(
            move |(_index, rpm_option), _deps| entered_rpm_setter.set(rpm_option),
            (),
        )
    };

    let determination_paste_ref = use_node_ref();
    let on_determination_paste = {
        let determination_paste_ref = determination_paste_ref.clone();
        let determinations_setter = determinations_state.setter();
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
                determinations_setter.set(grid);
            },
            (),
        )
    };

    let series_picker = {
        let set_picked_fan_series = picked_fan_series_state.setter();
        html! {
            <FanSeriesPicker
                selection={picked_fan_series_state.deref().clone()}
                no_selection_label={"--"}
                on_select={
                   move |value| set_picked_fan_series.set(value)
            }
            />
        }
    };

    let size_picker = {
        let picked_fan_size_setter = picked_fan_size_state.setter();
        let fan_series_id = picked_fan_series_state.deref().clone().map(|fs| fs.id);
        let option_predicate = use_callback(
            |fs: FanSize<()>, fan_series_id| match fan_series_id {
                Some(fan_series_id) => &fs.fan_series_id == fan_series_id,
                None => true,
            },
            fan_series_id,
        );
        html! {
            <FanSizePicker
                option_predicate={option_predicate}
                selection={(*picked_fan_size_state).clone()}
                no_selection_label={"--"}
                on_select={move |s| picked_fan_size_setter.set(s)}
            />
        }
    };

    match maybe_report {
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
                            value={(*entered_rpm_state).clone()}
                            index={0}
                            onchange={on_rpm_input_change}
                        />
                    </div>
                    <label><h2>{"Determination Points"}</h2></label>
                    <A12010DeterminationTable
                        fields={(*determinations_state).clone()}
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

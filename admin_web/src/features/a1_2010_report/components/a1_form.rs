use std::ops::Deref;
use std::rc::Rc;

use loquat_common::api::a1_2010_report::UpdateBody;

use yew::prelude::*;

use loquat_common::models::{
    A1Standard2010Determination, A1Standard2010Report, FanSeries, FanSize,
};

use crate::common::components::determination_table::TaggedInput;
use crate::common::components::{DeterminationsPasteTextArea, FanSeriesAndSizePicker};
use crate::features::a1_2010_report::components::A12010DeterminationTable;

#[derive(Debug, Properties, PartialEq)]
pub struct A1FormProps {
    pub report_id: Option<AttrValue>,
    pub maybe_report: Option<A1Standard2010Report<FanSize<FanSeries<()>>>>,
    pub on_valid_entry: Callback<UpdateBody>,
    pub on_submit_click: Callback<MouseEvent>,
}

#[derive(Debug, Clone, PartialEq)]
struct UpdateBodyErrors {
    size_errs: Vec<String>,
    rpm_errs: Vec<String>,
    determination_errs: Vec<[Rc<Vec<String>>; 3]>,
}

#[function_component]
pub fn A1Form(
    A1FormProps {
        report_id,
        maybe_report,
        on_valid_entry,
        on_submit_click,
    }: &A1FormProps,
) -> Html {
    let report_id_state: UseStateHandle<String> = {
        let report_id = report_id.clone();
        use_state(move || report_id.map_or("".to_string(), |id| id.to_string()))
    };
    let picked_fan_series_state: UseStateHandle<Option<FanSeries<()>>> = use_state(|| None);
    let picked_fan_size_state: UseStateHandle<Option<FanSize<()>>> = use_state(|| None);
    let entered_rpm_state: UseStateHandle<String> = use_state(|| "".to_string());
    let determinations_state: UseStateHandle<Vec<[String; 3]>> = use_state(Vec::new);

    let parsed_fan_size_id: Rc<Result<String, Vec<String>>> = use_memo(
        |picked_fan_size| match picked_fan_size {
            Some(fan_size) => Ok(fan_size.id.clone()),
            None => Err(vec!["You must select a fan size for the report".to_string()]),
        },
        picked_fan_size_state.deref().clone(),
    );

    let parsed_rpm: Rc<Result<f64, Vec<String>>> = use_memo(
        |entered_rpm: &String| match entered_rpm.deref().parse::<f64>() {
            Ok(value) => {
                if value <= 0.0 {
                    Err(vec!["The fan speed must be positive".to_string()])
                } else {
                    Ok(value)
                }
            }
            Err(_) => {
                if entered_rpm.is_empty() {
                    Err(vec!["You must enter a fan RPM for the test".to_string()])
                } else {
                    Err(vec!["You must enter a valid number".to_string()])
                }
            }
        },
        entered_rpm_state.deref().clone(),
    );

    let parsed_rpm_errs_rc = use_memo(
        |parsed_rpm| match parsed_rpm.as_ref() {
            Ok(_) => Vec::new(),
            Err(errs) => errs.clone(),
        },
        Rc::clone(&parsed_rpm),
    );

    let parsed_determinations =
        use_memo(parse_determenations, determinations_state.deref().clone());

    let fan_size_errs_rc = use_memo(
        |parsed_fan_size_id| match (*parsed_fan_size_id).as_ref() {
            Ok(_) => Vec::new(),
            Err(errs) => errs.clone(),
        },
        Rc::clone(&parsed_fan_size_id),
    );

    let parsed_update_body: Rc<Result<UpdateBody, UpdateBodyErrors>> = use_memo(
        |(parsed_fan_size_id, parsed_rpm, parsed_determinations)| {
            let parses = (
                parsed_fan_size_id.as_ref(),
                (parsed_rpm.as_ref()),
                (parsed_determinations.as_ref()),
            );
            if let (Ok(fan_size_id), Ok(fan_rpm), Ok(determinations)) = parses {
                let id: String = report_id_state.to_string();
                Ok(UpdateBody {
                    id,
                    determinations: determinations.clone(),
                    fan_rpm: *fan_rpm,
                    fan_size_id: fan_size_id.clone(),
                })
            } else {
                Err(UpdateBodyErrors {
                    size_errs: parsed_fan_size_id
                        .as_ref()
                        .clone()
                        .err()
                        .unwrap_or_default(),
                    rpm_errs: parsed_rpm.as_ref().clone().err().unwrap_or_default(),
                    determination_errs: parsed_determinations
                        .as_ref()
                        .clone()
                        .err()
                        .unwrap_or_default(),
                })
            }
        },
        (
            Rc::clone(&parsed_fan_size_id),
            Rc::clone(&parsed_rpm),
            Rc::clone(&parsed_determinations),
        ),
    );

    use_effect_with_deps(
        {
            let on_valid_entry = on_valid_entry.clone();
            move |parsed_update_body: &Rc<Result<UpdateBody, UpdateBodyErrors>>| {
                match parsed_update_body.as_ref() {
                    Ok(value) => on_valid_entry.emit(value.clone()),
                    Err(_) => (),
                };
                || {}
            }
        },
        Rc::clone(&parsed_update_body),
    );

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

    let on_report_id_change = {
        let report_id_setter = report_id_state.setter();
        use_callback(
            move |(_index, report_id), _deps| report_id_setter.set(report_id),
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

    let on_dets_input_change = {
        let determinations_table_setter = determinations_state.setter();
        use_callback(
            move |data: [[String; 3]; 10], _dets| determinations_table_setter.set(data.to_vec()),
            (),
        )
    };

    let on_determinations_extracted = {
        let determinations_setter = determinations_state.setter();
        use_callback(
            move |dets: Vec<[String; 3]>, _deps| determinations_setter.set(dets),
            (),
        )
    };

    let saved_size = maybe_report.as_ref().map(|report| {
        let (fan_size, _fan_series): (FanSize<()>, FanSeries<()>) = report.fan_size.clone().into();
        fan_size
    });

    let det_errs = use_memo(
        |parsed_determinations| (**parsed_determinations).clone().err().unwrap_or_default(),
        Rc::clone(&parsed_determinations),
    );

    html! {

                <form>
                    <h2>{"Test Details"}</h2>
                    <div style="display: grid; grid-template-columns: auto auto; width: fit-content; column-gap: 8px; row-gap: 4px;">
                        <label>{"Report ID"}</label>
                        <TaggedInput<()>
                            value={(*report_id_state).clone()}
                            tag={()}
                            onchange={on_report_id_change}
                        />
                        <FanSeriesAndSizePicker
                            size_errs={fan_size_errs_rc}
                            {saved_size}
                            {picked_fan_series_state}
                            {picked_fan_size_state}
                        />
                        <label>{"Test RPM"}</label>
                        <TaggedInput<()>
                            errs={parsed_rpm_errs_rc}
                            value={(*entered_rpm_state).clone()}
                            tag={()}
                            onchange={on_rpm_input_change}
                        />
                    </div>
                    <label><h2>{"Determination Points"}</h2></label>
                    <A12010DeterminationTable
                        fields={(*determinations_state).clone()}
                        child_errs={det_errs}
                        onchange={on_dets_input_change}
                    />
                    <label><h3>{"Quick Paste Determination Points"}</h3></label>
                    <DeterminationsPasteTextArea<3,10>
                        on_extracted={on_determinations_extracted}
                        cols_to_extract={[3,4,5]}
                        expected_row_length={9}
                        expected_headers={vec![
                            "Det. No. P t P v P s Q H K p η t η s",
                            "(in. wg) (in. wg) (in. wg) (cfm) (hp) - (%) (%)"
                        ]}
                    />
                    <button
                    //disabled={parsed_update_body.clone().is_err()}
                    onclick={on_submit_click}>{"Save"}</button>
                </form>

    }
}

fn parse_determenations(
    determinations_state: &Vec<[String; 3]>,
) -> Result<Vec<A1Standard2010Determination>, Vec<[Rc<Vec<String>>; 3]>> {
    let parsed_rows: Vec<Result<[f64; 3], [Rc<Vec<String>>; 3]>> = determinations_state
        .deref()
        .iter()
        .map(|det| {
            let det_attempt: [Result<f64, Rc<Vec<String>>>; 3] = det.clone().map(|x| {
                x.parse::<f64>().map_err(|_| {
                    Rc::new(if x.is_empty() {
                        vec!["All determination point values must be entered".to_string()]
                    } else {
                        vec!["You must enter a valid number".to_string()]
                    })
                })
            });
            if det_attempt.iter().all(|d| d.is_ok()) {
                Ok(det_attempt.map(|d| d.ok().unwrap()))
            } else {
                Err(det_attempt.map(|a| a.err().unwrap_or_default()))
            }
        })
        .collect::<Vec<_>>();

    if parsed_rows.iter().all(|d| d.is_ok()) {
        Ok(parsed_rows
            .into_iter()
            .map(|d| {
                let [static_pressure, cfm, brake_horsepower] = d.ok().unwrap();
                A1Standard2010Determination {
                    static_pressure,
                    cfm,
                    brake_horsepower,
                }
            })
            .collect())
    } else {
        Err(parsed_rows
            .into_iter()
            .map(|row| row.err().unwrap_or_default())
            .collect::<Vec<_>>())
    }
}

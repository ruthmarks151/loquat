use std::ops::Deref;
use std::rc::Rc;

use loquat_common::api::a1_2010_report::UpdateBody;

use yew::prelude::*;
use yewdux::prelude::use_store;

use loquat_common::models::{
    A1Standard2010Determination, A1Standard2010Report, FanSeries, FanSize,
};

use crate::api::store::Store as ApiStore;
use crate::common::components::determination_table::TaggedInput;
use crate::common::components::{DeterminationsPasteTextArea, FanSeriesAndSizePicker};
use crate::features::a1_2010_report::components::{A12010DeterminationTable, A1FanPlot};
use crate::store::select_a1_report;
use crate::{
    api::store::{ApiRequestAction, GetParameters, Gettable},
    store::use_app_store_selector_with_deps,
};

#[derive(Properties, PartialEq)]
pub struct EditA1PageProps {
    pub id: Option<AttrValue>,
}

#[function_component]
pub fn EditA1Page(props: &EditA1PageProps) -> Html {
    let (_state, api_dispatch) = use_store::<ApiStore>();

    let report_id = props.id.as_ref().map(|id| id.replace("%20", " "));
    let report_id_state: UseStateHandle<String> = {
        let report_id = report_id.clone();
        use_state(move || report_id.unwrap_or("".to_string()))
    };
    let picked_fan_series_state: UseStateHandle<Option<FanSeries<()>>> = use_state(|| None);
    let picked_fan_size_state: UseStateHandle<Option<FanSize<()>>> = use_state(|| None);
    let entered_rpm_state: UseStateHandle<String> = use_state(|| "".to_string());
    let determinations_state: UseStateHandle<Vec<[String; 3]>> = use_state(Vec::new);

    type FanSizeIdParse = Result<String, Vec<String>>;
    type RpmParse = Result<f64, Vec<String>>;

    #[derive(Debug, Clone, PartialEq)]
    struct UpdateBodyErrors {
        size_errs: Vec<String>,
        rpm_errs: Vec<String>,
        determination_errs: DeterminationsParseErr,
    }

    let parsed_fan_size_id: Rc<FanSizeIdParse> = use_memo(
        |picked_fan_size| match picked_fan_size {
            Some(fan_size) => Ok(fan_size.id.clone()),
            None => Err(vec!["You must select a fan size for the report".to_string()]),
        },
        picked_fan_size_state.deref().clone(),
    );

    let parsed_rpm: Rc<RpmParse> = use_memo(
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

    let parsed_determinations: Rc<DeterminationsParse> =
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

    let maybe_report: Option<A1Standard2010Report<FanSize<FanSeries<()>>>> =
        use_app_store_selector_with_deps(select_a1_report, report_id.clone());

    use_effect_with_deps(
        {
            let api_dispatch = api_dispatch.clone();
            move |report_id: &Option<String>| {
                if let Some(id) = report_id {
                    api_dispatch.apply(ApiRequestAction::Get(
                        GetParameters {
                            ignore_cache: false,
                        },
                        Gettable::A1Report { id: id.clone() },
                    ));
                }
                || {}
            }
        },
        report_id,
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

    let handle_save_callback = {
        use_callback(
            |_evt: MouseEvent, (dispatch, parsed_update_body_ref)| {
                if let Ok(update_body) = (*parsed_update_body_ref).as_ref().clone() {
                    dispatch.apply(ApiRequestAction::Get(
                        GetParameters { ignore_cache: true },
                        Gettable::PutA12010Report { body: update_body },
                    ))
                }
            },
            (api_dispatch, Rc::clone(&parsed_update_body)),
        )
    };

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

    let (header, maybe_test_id_input) = match &maybe_report {
        None => {
            let test_id_input = html! {
                <>
                    <label>{"Report ID"}</label>
                    <TaggedInput<()>
                        value={(*report_id_state).clone()}
                        tag={()}
                        onchange={on_report_id_change}
                    />
                </>
            };
            (html! {<h1>{"New A1 Report"}</h1>}, Some(test_id_input))
        }
        Some(report) => (html! {<h1>{"Test No. "}{ report.id.to_owned() }</h1>}, None),
    };

    let det_errs = use_memo(
        |parsed_determinations| {
            parsed_determinations
                .as_ref()
                .clone()
                .err()
                .unwrap_or_default()
        },
        Rc::clone(&parsed_determinations),
    );

    let maybe_points_to_render = use_memo(
        |parsed_update_body| {
            parsed_update_body
                .as_ref()
                .clone()
                .map(|u| Some(u.determinations))
                .unwrap_or(maybe_report.map(|r| r.determinations))
        },
        Rc::clone(&parsed_update_body),
    );

    let plot_html = match maybe_points_to_render.as_ref() {
        Some(fc) => html! { <A1FanPlot points={fc.clone()} /> },
        None => html! { <p>{"Once you enter a complete fan curve, you'll see it here"}</p> },
    };

    html! {
        <>
            {header}
            <div style="display: flex; flex-direction: row;">
                <form>
                    <h2>{"Test Details"}</h2>
                    <div style="display: grid; grid-template-columns: auto auto; width: fit-content; column-gap: 8px; row-gap: 4px;">
                        {maybe_test_id_input}
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
                    onclick={handle_save_callback}>{"Save"}</button>
                </form>
                <div style="flex-grow: 1">
                    {plot_html}
                </div>
            </div>
        </>
    }
}

type DeterminationValueErr = Rc<Vec<String>>;
type DeterminationRowErr = [DeterminationValueErr; 3];
type DeterminationsParseErr = Vec<DeterminationRowErr>;
type DeterminationsParse = Result<Vec<A1Standard2010Determination>, DeterminationsParseErr>;

fn parse_determenations(determinations_state: &Vec<[String; 3]>) -> DeterminationsParse {
    let parsed_rows: Vec<Result<[f64; 3], [DeterminationValueErr; 3]>> = determinations_state
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

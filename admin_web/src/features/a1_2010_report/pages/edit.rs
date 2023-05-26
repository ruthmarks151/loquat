use yew::prelude::*;

use crate::features::a1_2010_report::components::{A1FanPlot, A1Form};
use crate::features::a1_2010_report::hooks::{use_a1_form_controller, A1FormHookRes};

#[derive(Properties, PartialEq)]
pub struct EditA1PageProps {
    pub id: AttrValue,
}

#[function_component]
pub fn EditA1Page(props: &EditA1PageProps) -> Html {
    let report_id = props.id.replace("%20", " ");

    let A1FormHookRes {
        on_valid_entry,
        maybe_report,
        maybe_points_to_render,
        on_submit_click,
    } = use_a1_form_controller(Some(report_id.clone()));

    let plot_html = match maybe_points_to_render.as_ref() {
        Some(fc) => html! { <A1FanPlot points={fc.clone()} /> },
        None => html! { <p>{"Once you correct the fan curve you'll see it here"}</p> },
    };

    html! {
        <>
            <h1>{"Test No. "}{ report_id.clone() }</h1>
            <div style="display: flex; flex-direction: row;">
                <A1Form
                    {report_id}
                    {maybe_report}
                    {on_valid_entry}
                    {on_submit_click}
                />
                <div style="flex-grow: 1">
                    {plot_html}
                </div>
            </div>
        </>
    }
}

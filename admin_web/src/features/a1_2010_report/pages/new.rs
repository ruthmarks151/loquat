use yew::prelude::*;

use crate::features::a1_2010_report::components::{A1FanPlot, A1Form};
use crate::features::a1_2010_report::hooks::{use_a1_form_controller, A1FormHookRes};

#[function_component]
pub fn NewA1Page() -> Html {
    let A1FormHookRes {
        on_valid_entry,
        maybe_report: _,
        maybe_points_to_render,
        on_submit_click,
    } = use_a1_form_controller(None);

    let plot_html = match maybe_points_to_render.as_ref() {
        Some(fc) => html! { <A1FanPlot points={fc.clone()} /> },
        None => html! { <p>{"Once you enter a complete fan curve, you'll see it here"}</p> },
    };

    html! {
        <>
            <h1>{"New A1 Report"}</h1>
            <div style="display: flex; flex-direction: row;">
                <A1Form
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

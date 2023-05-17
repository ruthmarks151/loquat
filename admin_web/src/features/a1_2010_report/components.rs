use std::rc::Rc;

use web_sys::{HtmlInputElement, InputEvent};
use yew::{function_component, html, use_callback, use_node_ref, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct DeterminationTableProps<const COL_COUNT: usize> {
    pub headers: [String; COL_COUNT],
    pub rows: Vec<[Option<f64>; COL_COUNT]>,
    pub onchange: Callback<(usize, usize, Option<f64>), ()>,
}

#[function_component]
pub fn DeterminationTable<const COL_COUNT: usize>(
    DeterminationTableProps {
        headers,
        rows,
        onchange,
    }: &DeterminationTableProps<COL_COUNT>,
) -> Html {
    let header_html: Html = headers
        .clone()
        .map(|header_val| html! { <th>{header_val}</th> })
        .into_iter()
        .collect();
    html! {
      <table>
        <tr>
          <th>
            {"Det. No."}
          </th>
          {header_html}
        </tr>
        {rows.iter().enumerate().map(|(row_index, d)| html! { <DeterminationTableRow<COL_COUNT> {onchange} {row_index} values={d.clone()} /> } ).collect::<Html>()}
      </table>
    }
}

#[derive(Properties, PartialEq)]
pub struct DeterminationTableRowProps<const COL_COUNT: usize> {
    pub values: [Option<f64>; COL_COUNT],
    pub row_index: usize,
    pub onchange: Callback<(usize, usize, Option<f64>), ()>,
}

#[function_component]
pub fn DeterminationTableRow<const COL_COUNT: usize>(
    DeterminationTableRowProps {
        values,
        row_index,
        onchange,
    }: &DeterminationTableRowProps<COL_COUNT>,
) -> Html {
    let handle_change: Callback<(usize, Option<f64>)> = use_callback(
        move |(col_index, value), (row_index_ref, onchange_ref)| {
            onchange_ref.emit((row_index_ref.clone(), col_index, value))
        },
        (row_index.clone(), onchange.clone()),
    );

    let row_html: Html = values
        .iter()
        .enumerate()
        .map(|(index, val)| {
            html! {
              <td>
                  <FloatInput onchange={handle_change.clone()} {index} value={val} />
              </td>
            }
        })
        .collect();
    html! {
      <tr>
        <td>
          {row_index + 1}
        </td>
        {row_html}
      </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct FloatInputProps {
    pub value: Option<f64>,
    pub index: usize,
    pub onchange: Callback<(usize, Option<f64>), ()>,
}

#[function_component]
pub fn FloatInput(
    FloatInputProps {
        value,
        index,
        onchange,
    }: &FloatInputProps,
) -> Html {
    let input_ref = use_node_ref();

    let onblur = use_callback(
        move |_evt, (index_ref, input_ref, onchange_ref)| {
            let input = input_ref
                .cast::<HtmlInputElement>()
                .expect("input_ref not attached to input element");

            let val: Option<f64> = input.value().parse().ok();
            onchange_ref.emit((index_ref.clone(), val.clone()).clone());
        },
        (index.clone(), input_ref.clone(), onchange.clone()),
    );
    let style = if value.is_some() {
        ""
    } else {
        "border-color: red;"
    };

    html! {
      <input {style} ref={input_ref} value={value.map(|v| v.to_string())} onblur={onblur.clone()} />
    }
}

use web_sys::HtmlInputElement;
use yew::{function_component, html, use_callback, use_node_ref, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct DeterminationTableProps<const COL_COUNT: usize, const ROW_COUNT: usize> {
    pub headers: [String; COL_COUNT],
    pub rows: [[String; COL_COUNT]; ROW_COUNT],
    pub onchange: Callback<(usize, usize, String), ()>,
}

#[function_component]
pub fn DeterminationTable<const COL_COUNT: usize, const ROW_COUNT: usize>(
    DeterminationTableProps {
        headers,
        rows,
        onchange,
    }: &DeterminationTableProps<COL_COUNT, ROW_COUNT>,
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
    pub values: [String; COL_COUNT],
    pub row_index: usize,
    pub onchange: Callback<(usize, usize, String), ()>,
}

#[function_component]
pub fn DeterminationTableRow<const COL_COUNT: usize>(
    DeterminationTableRowProps {
        values,
        row_index,
        onchange,
    }: &DeterminationTableRowProps<COL_COUNT>,
) -> Html {
    let handle_change: Callback<(usize, String)> = use_callback(
        move |(col_index, value), (row_index_ref, onchange_ref)| {
            onchange_ref.emit((row_index_ref.clone(), col_index, value))
        },
        (row_index.clone(), onchange.clone()),
    );

    let row_html: Html = values
        .into_iter()
        .enumerate()
        .map(|(index, val)| {
            html! {
              <td>
                  <TaggedInput<usize> onchange={handle_change.clone()} tag={index} value={val.clone()} />
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
pub struct TaggedInputProps<T: Clone + Eq + 'static> {
    pub value: String,
    pub tag: T,
    pub onchange: Callback<(T, String), ()>,
}

#[function_component]
pub fn TaggedInput<T: Clone + Eq + 'static>(
    TaggedInputProps {
        value,
        tag,
        onchange,
    }: &TaggedInputProps<T>,
) -> Html {
    let input_ref = use_node_ref();

    let onblur = use_callback(
        move |_evt, (tag_ref, input_ref, onchange_ref)| {
            let input = input_ref
                .cast::<HtmlInputElement>()
                .expect("input_ref not attached to input element");

            onchange_ref.emit((tag_ref.clone(), input.value()).clone());
        },
        (tag.clone(), input_ref.clone(), onchange.clone()),
    );

    html! {
      <input ref={input_ref} value={value.clone()} onblur={onblur.clone()} />
    }
}

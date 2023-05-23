use std::{fmt::Debug, iter};

use yew::{function_component, html, use_callback, Callback, Html, Properties};

use crate::common::components::DeterminationTable;

#[derive(Properties, PartialEq)]
pub struct A12010DeterminationTableProps {
    pub fields: Vec<[String; 3]>,
    pub child_errs: Vec<[Vec<String>; 3]>,
    pub onchange: Callback<[[String; 3]; 10]>,
}

fn to_filled_table<T: Clone + Debug>(
    given_rows: &Vec<[T; 3]>,
    empty: impl Fn() -> T,
) -> [[T; 3]; 10] {
    given_rows
        .iter()
        .chain(iter::repeat(&[empty(), empty(), empty()]))
        .take(10)
        .cloned()
        .collect::<Vec<[T; 3]>>()
        .try_into()
        .unwrap()
}

#[function_component]
pub fn A12010DeterminationTable(props: &A12010DeterminationTableProps) -> Html {
    let rows = to_filled_table(&props.fields, || "".to_string());
    let child_errs = to_filled_table(&props.child_errs, || vec![]);
    let on_determination_value_change: Callback<(usize, usize, String)> = use_callback(
        move |(row_index, col_index, value): (usize, usize, String), (rows, onchange)| {
            let mut rows = rows.clone();
            rows[row_index][col_index] = value;

            onchange.emit(rows);
        },
        (rows.clone(), props.onchange.clone()),
    );

    let headers_lables: [String; 3] = [
        "Static Pressure (in. wg)".to_string(),
        "Flow Rate (cfm)".to_string(),
        "Brake Horsepower (hp)".to_string(),
    ];

    html! {
        <DeterminationTable<3, 10>
            headers={headers_lables}
            onchange={on_determination_value_change}
            {rows}
            {child_errs}
        />
    }
}

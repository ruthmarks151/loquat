use std::iter;

use loquat_common::models::A1Standard2010Determination;
use yew::{function_component, html, use_callback, Callback, Html, Properties};

use crate::common::components::DeterminationTable;

#[derive(Properties, PartialEq)]
pub struct A12010DeterminationTableProps {
    pub fields: Vec<[String; 3]>,
    pub onchange: Callback<[[String; 3]; 10]>,
}

fn to_filled_table(given_rows: &Vec<[String; 3]>) -> [[String; 3]; 10] {
    log::info!("Filling up {:?}", given_rows);
    given_rows
        .iter()
        .chain(iter::repeat(&[
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ]))
        .take(10)
        .cloned()
        .collect::<Vec<[String; 3]>>()
        .try_into()
        .unwrap()
}

#[function_component]
pub fn A12010DeterminationTable(props: &A12010DeterminationTableProps) -> Html {
    let rows = to_filled_table(&props.fields);

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
        />
    }
}

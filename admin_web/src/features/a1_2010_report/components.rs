use std::{fmt::Debug, iter, rc::Rc};

use loquat_common::models::A1Standard2010Determination;
use yew::{
    function_component, html, platform::spawn_local, use_callback, use_effect_with_deps, Callback,
    Html, Properties,
};

use crate::common::components::DeterminationTable;

#[derive(Properties, PartialEq)]
pub struct A12010DeterminationTableProps {
    pub fields: Vec<[String; 3]>,
    pub child_errs: Rc<Vec<[Rc<Vec<String>>; 3]>>,
    pub onchange: Callback<[[String; 3]; 10]>,
}

fn to_filled_table<T: Clone + Debug>(given_rows: &[[T; 3]], empty: impl Fn() -> T) -> [[T; 3]; 10] {
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
    let child_errs = to_filled_table(&props.child_errs, || Rc::new(Vec::new()));
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

use plotly::{
    common::{AxisSide, Marker, MarkerSymbol, Mode},
    layout::{Axis, Legend, RangeMode},
    Layout, Plot, Scatter,
};

#[derive(Properties, PartialEq)]
pub struct A1FanPlotProps {
    pub points: Vec<A1Standard2010Determination>,
}

#[function_component]
pub fn A1FanPlot(A1FanPlotProps { points }: &A1FanPlotProps) -> Html {
    use_effect_with_deps(
        move |points| {
            let id = "plot-div";
            let mut plot = Plot::new();

            let max_sp = points
                .iter()
                .map(|p| p.static_pressure)
                .reduce(f64::max)
                .unwrap_or_default();
            let max_bhp = points
                .iter()
                .map(|p| p.brake_horsepower)
                .reduce(f64::max)
                .unwrap_or_default();

            let err = (max_bhp - max_sp) / (max_bhp + max_sp);
            let shared_max = if err < 0.1 {
                Some(f64::max(max_bhp, max_sp).ceil())
            } else {
                None
            };
            let pressure_curve = Scatter::new(
                points.iter().map(|p| p.cfm).collect(),
                points.iter().map(|p| p.static_pressure).collect(),
            )
            .name("Static Pressure (In. Wg.)")
            .mode(Mode::LinesMarkers)
            .marker(Marker::new().symbol(MarkerSymbol::SquareOpen))
            .y_axis("y");

            let bhp_curve = Scatter::new(
                points.iter().map(|p| p.cfm).collect(),
                points.iter().map(|p| p.brake_horsepower).collect(),
            )
            .name("Power Input (HP)")
            .mode(Mode::LinesMarkers)
            .marker(Marker::new().symbol(MarkerSymbol::CircleOpen))
            .y_axis("y2");

            let layout = Layout::new()
                .title("A1 Fan Curve".into())
                .legend(Legend::new().x(0.1).y(0.0))
                .x_axis(Axis::new().title("Airflow (cfm)".into()))
                .y_axis(
                    Axis::new()
                        .range_mode(RangeMode::ToZero)
                        .title("Static Pressure (In. Wg.)".into())
                        .side(AxisSide::Left),
                )
                .y_axis({
                    let y = Axis::new().title("Airflow (cfm)".into());
                    if let Some(shared_max) = shared_max {
                        y.range(vec![0., shared_max])
                    } else {
                        y
                    }
                })
                .y_axis2({
                    let y = Axis::new()
                        .title("Power Input (HP)".into())
                        .range_mode(RangeMode::ToZero)
                        .show_line(true)
                        .overlaying("y")
                        .side(AxisSide::Right);
                    if let Some(shared_max) = shared_max {
                        y.range(vec![0., shared_max])
                    } else {
                        y
                    }
                });
            plot.set_layout(layout);
            plot.add_trace(pressure_curve);
            plot.add_trace(bhp_curve);
            spawn_local(async move {
                plotly::bindings::new_plot(id, &plot).await;
            });
            || ()
        },
        points.clone(),
    );

    html! {
        <div id="plot-div"></div>
    }
}

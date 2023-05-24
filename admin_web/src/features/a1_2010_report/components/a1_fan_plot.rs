use loquat_common::models::A1Standard2010Determination;
use plotly::{
    common::{AxisSide, Marker, MarkerSymbol, Mode},
    layout::{Axis, Legend, RangeMode},
    Layout, Plot, Scatter,
};
use yew::{
    function_component, html, platform::spawn_local, use_effect_with_deps, Html, Properties,
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

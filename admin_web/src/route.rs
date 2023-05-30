use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    // Fan Series
    #[at("/fan_series")]
    IndexFanSerieses,
    #[at("/fan_series/new")]
    NewFanSeriesPage,
    #[at("/fan_series/:id")]
    GetFanSeries { id: String },
    #[at("/fan_series/:id/edit")]
    EditFanSeries { id: String },
    // Fan Sizes
    #[at("/fan_size/:id")]
    GetFanSize { id: String },
    #[at("/a1_report/new")]
    NewA1Report,
    #[at("/a1_report/:id/edit")]
    EditA1Report { id: String },
}

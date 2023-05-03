use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    // Fan Series
    #[at("/fan_series")]
    IndexFanSerieses,
    #[at("/fan_series/:id")]
    GetFanSeries { id: String },
    // Fan Sizes
    #[at("/fan_size/:id")]
    GetFanSize { id: String },
}

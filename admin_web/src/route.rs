use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/fan_series")]
    IndexFanSerieses,
    #[at("/fan_series/:id")]
    GetFanSeries { id: String },
}

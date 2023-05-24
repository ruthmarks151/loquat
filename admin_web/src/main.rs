use yew::prelude::*;
use yew_router::prelude::*;

use loquat_admin_web::{
    features::fan_series::pages::{IndexFanSeriesPage, ReadFanSeriesPage},
    features::{
        a1_2010_report::pages::{EditA1Page, NewA1Page},
        fan_size::pages::ReadFanSizePage,
    },
    route::Route,
};

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Hello Frontend" }</h1> },
        Route::IndexFanSerieses => html! { <IndexFanSeriesPage /> },
        Route::GetFanSeries { id } => html! { <ReadFanSeriesPage id={id} /> },
        Route::GetFanSize { id } => html! { <ReadFanSizePage id={id} /> },
        Route::EditA1Report { id } => html! { <EditA1Page id={id} /> },
        Route::NewA1Report => html! { <NewA1Page /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}

use yew::prelude::*;
use yew_router::prelude::*;

use loquat_admin_web::{
    features::fan_series::pages::{IndexFanSeriesPage, ReadFanSeriesPage},
    features::fan_size::pages::ReadFanSizePage,
    route::Route,
};

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Hello Frontend" }</h1> },
        Route::IndexFanSerieses => html! { <IndexFanSeriesPage /> },
        Route::GetFanSeries { id } => html! { <ReadFanSeriesPage id={id} /> },
        Route::GetFanSize { id } => html! { <ReadFanSizePage id={id} /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}

use gloo_net::http::Request;
use loquat_common::models::Fan;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/fans")]
    IndexFans,
    #[at("/fans/:id")]
    GetFan { id: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Hello Frontend" }</h1> },
        Route::IndexFans => html! { <IndexFans /> },
        Route::GetFan { id } => html! { <GetFan id={id} /> },
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

#[derive(Properties, PartialEq)]
struct GetFanProps {
    id: String,
}

#[function_component(GetFan)]
fn get_fan(props: &GetFanProps) -> Html {
    let data = use_state(|| None);

    // Request `/api/hello` once
    {
        let data = data.clone();
        let fan_id = props.id.clone();
        let req_url = format!("/api/fans/{}", fan_id);
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get(req_url.as_str()).send().await.unwrap();
                    let result: Result<Fan, String> = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            resp.json().await.map_err(|err| err.to_string())
                        }
                    };
                    data.set(Some(result));
                });
            }

            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div>
                    <h1>{"Fan Detail"}</h1>
                    {"id: "} {data.id.to_owned()} <br/>
                    {" Name: "} {data.name.to_owned()}
                </div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }
}

#[function_component(IndexFans)]
fn get_fan() -> Html {
    let data = use_state(|| None);

    // Request `/api/hello` once
    {
        let data = data.clone();
        let req_url = "/api/fans".to_owned();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get(req_url.as_str()).send().await.unwrap();
                    let result: Result<Vec<Fan>, String> = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            resp.json().await.map_err(|err| err.to_string())
                        }
                    };
                    data.set(Some(result));
                });
            }

            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div>
                    <h1>{"Fan List"}</h1>
                    <ul>
                        { data.iter().map(|fan| html! { <li>
                            <Link<Route> to={Route::GetFan { id: fan.id.clone() }}> {fan.name.clone()}  </Link<Route>>




                            </li>


                        } ).collect::<Vec<_>>() }
                    </ul>
                </div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}

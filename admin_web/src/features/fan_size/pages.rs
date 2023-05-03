pub mod pages {
    pub mod read {
        use yew::{platform::spawn_local, prelude::*};
        use yew_router::prelude::Link;

        use super::super::super::api::get_fan_size;
        use crate::route::Route;

        #[derive(Properties, PartialEq)]
        pub struct ReadFanSizePageProps {
            pub id: String,
        }

        #[function_component]
        pub fn ReadFanSizePage(props: &ReadFanSizePageProps) -> Html {
            let data = use_state(|| None);
            {
                let data = data.clone();
                let id = props.id.clone();
                use_effect(move || {
                    if data.is_none() {
                        spawn_local(async move {
                            data.set(Some(get_fan_size(id).await));
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
                    html! { <div>
                        <h1>{ data.fan_size.id.to_owned() }</h1>
                        <h2>
                            {"Diameter: "} {data.fan_size.diameter.clone()}
                        </h2>
                        <Link<Route> to={Route::GetFanSeries { id: data.fan_series.id.clone() }}>
                          <h2>
                              {"Series: "} {data.fan_series.id.clone()}
                          </h2>
                        </Link<Route>>
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
    }
}

pub use pages::read::ReadFanSizePage;

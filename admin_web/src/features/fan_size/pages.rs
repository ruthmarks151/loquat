pub mod pages {
    pub mod read {
        use std::rc::Rc;

        use loquat_common::models::fan_size::FanSize;
        use yew::{platform::spawn_local, prelude::*};
        use yew_router::prelude::Link;
        use yewdux::prelude::{use_selector_with_deps, use_store};

        use super::super::super::api::get_fan_size;
        use crate::{route::Route, store::{selectors::select_fan_size_by_id, FanStoreActions, FanStore}};

        #[derive(Properties, PartialEq)]
        pub struct ReadFanSizePageProps {
            pub id: String,
        }

        #[function_component]
        pub fn ReadFanSizePage(props: &ReadFanSizePageProps) -> Html {
            let (_state, dispatch) = use_store::<FanStore>();
            let id = props.id.clone();

            let format_id = id.clone().replace("%20", " ");
            let fan_size_option: Rc<Option<FanSize>> =
                use_selector_with_deps(select_fan_size_by_id, format_id);

            use_effect(move || {
                spawn_local(async move {
                    if let Ok(json_resp) = get_fan_size(id).await {
                        dispatch.apply(FanStoreActions::InsertFanSize(json_resp.fan_size));
                    }
                });
                || {}
            });

            match fan_size_option.as_ref() {
                None => {
                    html! {
                        <div>{"No server response"}</div>
                    }
                }
                Some(fan_size) => {
                    html! { <div>
                        <h1>{ fan_size.id.to_owned() }</h1>
                        <h2>
                            {"Diameter: "} {fan_size.diameter.clone()}
                        </h2>
                        <Link<Route> to={Route::GetFanSeries { id: fan_size.fan_series_id.clone() }}>
                          <h2>
                              {"Series: "} {fan_size.fan_series_id.clone()}
                          </h2>
                        </Link<Route>>
                      </div>
                    }
                }
            }
        }
    }
}

pub use pages::read::ReadFanSizePage;

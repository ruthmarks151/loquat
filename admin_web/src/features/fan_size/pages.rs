pub mod read {
    use std::rc::Rc;

    use loquat_common::models::{FanSeries, FanSize};
    use yew::prelude::*;
    use yew_router::prelude::Link;
    use yewdux::prelude::use_store;

    use crate::api::store::Store as ApiStore;
    use crate::{
        api::store::{ApiRequestAction, GetParameters, Gettable},
        route::Route,
        store::{select_fan_size_by_id, use_app_store_selector_with_deps},
    };

    #[derive(Properties, PartialEq)]
    pub struct ReadFanSizePageProps {
        pub id: String,
    }

    #[function_component]
    pub fn ReadFanSizePage(props: &ReadFanSizePageProps) -> Html {
        let (_state, dispatch) = use_store::<ApiStore>();
        let id = props.id.clone();

        let format_id = id.replace("%20", " ");
        let fan_size_option: Rc<Option<FanSize<FanSeries<()>>>> =
            use_app_store_selector_with_deps(select_fan_size_by_id, format_id.clone());

        use_effect_with_deps(
            move |_| {
                dispatch.apply(ApiRequestAction::Get(
                    GetParameters {
                        ignore_cache: false,
                    },
                    Gettable::FanSize { id: format_id },
                ));
                || {}
            },
            (),
        );

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
                        {"Diameter: "} {fan_size.diameter}
                        {"Fan Type: "} {fan_size.fan_series.fan_type.clone() }
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

pub use read::ReadFanSizePage;

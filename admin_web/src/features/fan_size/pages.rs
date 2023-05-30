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
                    <h1>
                        <Link<Route> to={Route::GetFanSeries { id: fan_size.fan_series_id.clone() }}>
                            {'\u{2b05}'} // Fat Left Arrow
                            {'\u{2002}'} // en-space
                        </Link<Route>>
                        { fan_size.id.to_owned() }
                    </h1>
                    <table>
                        <tr>
                            <td>{"Diameter: "}</td>
                            <td> {fan_size.diameter}</td>
                        </tr>
                        <tr>
                            <td>{"Fan Type: "}</td>
                            <td>{fan_size.fan_series.fan_type.clone() }</td>
                        </tr>
                        <tr>
                            <td>{"Series: "}</td>
                            <td>
                                <Link<Route> to={Route::GetFanSeries { id: fan_size.fan_series_id.clone() }}>
                                    {fan_size.fan_series_id.clone()}
                                </Link<Route>>
                            </td>
                        </tr>
                    </table>
                    
                    </div>
                }
            }
        }
    }
}

pub use read::ReadFanSizePage;

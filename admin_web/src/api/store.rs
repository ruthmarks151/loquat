use std::{collections::HashMap, future::Future, rc::Rc};

use instant::Instant;
use loquat_common::models::{A1Standard2010Report, FanSeries, FanSize};
use serde;
use yew::platform::spawn_local;
use yewdux::{
    prelude::{self, Dispatch},
    store::Reducer,
};

use crate::store::app_dispatch;

use super::{a1_report, fan_series, fan_size};

#[derive(Debug, Default, Clone, PartialEq, Eq, prelude::Store)]
pub struct Store {
    pub get_status: HashMap<Gettable, RequestStatuses>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RequestStatuses {
    #[default]
    Unfetched,
    Fetching(Instant),
    Refetching(Instant, Box<RequestStatuses>),
    Fetched(Instant),
    Error(Instant, String),
}

fn should_fetch(current_status: RequestStatuses, force: bool) -> bool {
    match current_status {
        RequestStatuses::Unfetched => true,
        RequestStatuses::Error(_, _) => true,

        // Think about the time for these?
        RequestStatuses::Fetching(_) => false,
        RequestStatuses::Refetching(_, _) => false,
        RequestStatuses::Fetched(_) => force,
    }
}

#[derive(Debug, Clone)]
pub struct GetParameters {
    pub ignore_cache: bool,
}

#[derive(Debug, Clone)]
pub enum ApiRequestAction {
    Get(GetParameters, Gettable),
}

#[derive(Debug, Clone)]
pub enum ApiResponseAction {
    RecieveFanSerieses(Vec<FanSeries<()>>),
    RecieveFanSeries(FanSeries<Vec<FanSize<()>>>),
    RecieveFanSizes(Vec<FanSize<()>>),
    RecieveFanSize(FanSize<FanSeries<()>>),
    RecieveA1Report(A1Standard2010Report<FanSize<FanSeries<()>>>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Gettable {
    FanSeriesesIndex,
    FanSeries {
        id: String,
    },
    FanSizesIndex,
    FanSize {
        id: String,
    },
    A1Report {
        id: String,
    },
    PutA12010Report {
        body: loquat_common::api::a1_2010_report::UpdateBody,
    },
    PostA12010Report {
        body: loquat_common::api::a1_2010_report::UpdateBody,
    },
    PutFanSeriesReport {
        body: loquat_common::api::fan_series::UpdateBody,
    },
    PostFanSeriesReport {
        body: loquat_common::api::fan_series::UpdateBody,
    },
}

impl Reducer<Store> for ApiRequestAction {
    fn apply(self, store: Rc<Store>) -> Rc<Store> {
        log::info!("{:#?}", self);
        match self {
            ApiRequestAction::Get(params, gettable) => {
                let status: RequestStatuses =
                    store.get_status.get(&gettable).cloned().unwrap_or_default();
                if !should_fetch(status, params.ignore_cache) {
                    return store;
                }
                match gettable.clone() {
                    Gettable::FanSeriesesIndex => handle_dispatches(
                        gettable,
                        fan_series::index(),
                        ApiResponseAction::RecieveFanSerieses,
                    ),
                    Gettable::FanSeries { id } => handle_dispatches(
                        gettable,
                        fan_series::get(id),
                        ApiResponseAction::RecieveFanSeries,
                    ),
                    Gettable::FanSizesIndex => handle_dispatches(
                        gettable,
                        fan_size::index(),
                        ApiResponseAction::RecieveFanSizes,
                    ),
                    Gettable::FanSize { id } => handle_dispatches(
                        gettable,
                        fan_size::get(id),
                        ApiResponseAction::RecieveFanSize,
                    ),
                    Gettable::A1Report { id } => handle_dispatches(
                        gettable,
                        a1_report::get(id),
                        ApiResponseAction::RecieveA1Report,
                    ),
                    Gettable::PutA12010Report { body } => handle_dispatches(
                        gettable,
                        a1_report::put(body),
                        ApiResponseAction::RecieveA1Report,
                    ),
                    Gettable::PostA12010Report { body } => handle_dispatches(
                        gettable,
                        a1_report::post(body),
                        ApiResponseAction::RecieveA1Report,
                    ),
                    Gettable::PutFanSeriesReport { body }  => handle_dispatches(
                        gettable,
                        fan_series::put(body),
                        ApiResponseAction::RecieveFanSeries,
                    ),
                    Gettable::PostFanSeriesReport { body }  => handle_dispatches(
                        gettable,
                        fan_series::post(body),
                        ApiResponseAction::RecieveFanSeries,
                    ),
                }
            }
        }
        store
    }
}

fn handle_dispatches<Fut, Resp, ActionFactory>(
    gettable: Gettable,
    fetching_future: Fut,
    action: ActionFactory,
) where
    ActionFactory: Fn(Resp) -> ApiResponseAction,
    ActionFactory: 'static,
    Fut: Future<Output = Result<gloo_net::http::Response, gloo_net::Error>> + 'static,
    Resp: for<'de> serde::de::Deserialize<'de>,
{
    let dispatch = Dispatch::<Store>::new();
    dispatch.reduce_mut(|s| {
        s.get_status
            .insert(gettable.clone(), RequestStatuses::Fetching(Instant::now()));
    });

    spawn_local(async move {
        let api_resp = fetching_future.await;

        if let Ok(resp) = api_resp {
            if !resp.ok() {
                let err_msg = format!(
                    "Error fetching data {} ({})",
                    resp.status(),
                    resp.status_text()
                );
                if resp.status() == 401 {
                    // not authed
                    log::warn!("401 unauthorized, redirecting");
                    web_sys::window().map(|w| w.location().set_pathname("/static/login.html"));
                }
                dispatch.reduce_mut(|s| {
                    s.get_status
                        .insert(gettable, RequestStatuses::Error(Instant::now(), err_msg));
                });
            } else {
                let json_parse = resp.json().await;
                if let Ok(obj) = json_parse {
                    app_dispatch(action(obj));
                    dispatch.reduce_mut(|s| {
                        s.get_status
                            .insert(gettable, RequestStatuses::Fetched(Instant::now()));
                    });
                } else if let Err(err) = json_parse {
                    dispatch.reduce_mut(|s| {
                        s.get_status.insert(
                            gettable,
                            RequestStatuses::Error(Instant::now(), err.to_string()),
                        );
                    });
                }
            }
        } else if let Err(error) = api_resp {
            dispatch.reduce_mut(|s| {
                s.get_status.insert(
                    gettable,
                    RequestStatuses::Error(Instant::now(), error.to_string()),
                );
            });
        }
    });
}

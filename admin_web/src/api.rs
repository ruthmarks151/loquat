// use instant::Instant;
// use std::{future::Future, rc::Rc};
// use yew::platform::spawn_local;
// use yewdux::{prelude::Dispatch, store::Reducer};

mod a1_report;
mod fan_series;
mod fan_size;
pub mod store;

struct FetchableData<DataType, ErrorType> {
    is_fetching: bool,
    error: Option<ErrorType>,
    data: DataType,
}

// fn fetch_and_store<Fut, Resp, Store: yewdux::prelude::Store, Action: Reducer<Store>>(
//     route: EndpointRequest,
//     fetching_future: Fut,
//     action: impl Fn(Resp) -> Action + 'static,
// ) where
//     Fut: Future<Output = Result<Resp, String>> + 'static,
// {
//     spawn_local(async move {
//         let api_resp = fetching_future.await;
//         if let Ok(json_resp) = api_resp {
//             let dispatch = Dispatch::<Store>::new();
//             let api_dispatch = Dispatch::<ApiStore>::new();

//             dispatch.apply(action(json_resp));
//             api_dispatch.reduce_mut(|s| {
//                 s.action_status
//                     .insert(route, RequestStatuses::Fetched(Instant::now()));
//             })
//         } else if let Err(err_msg) = api_resp {
//             s.action_status
//                     .insert(route, RequestStatuses::Error(Instant::now(), err_msg));
//         }
//     });
// }

// fn fetch_and_store_many<Fut, Resp: Sized, Store: yewdux::prelude::Store, Action: Reducer<Store>>(
//     route: FetchRoutes,
//     fetching: Fut,
//     action: impl Fn(Resp) -> Action + 'static,
// ) where
//     Fut: Future<Output = Result<Vec<Resp>, String>> + 'static,
// {
//     spawn_local(async move {
//         if let Ok(json_resp) = fetching.await {
//             let dispatch = Dispatch::<Store>::new();
//             let api_dispatch = Dispatch::<ApiStore>::new();

//             for item in json_resp {
//                 dispatch.apply(action(item));
//             }
//             api_dispatch.reduce_mut(|s| {
//                 s.action_status
//                     .insert(route, RequestStatuses::Fetched(Instant::now()));
//             })
//         }
//     });
// }

// // Common ///////////////

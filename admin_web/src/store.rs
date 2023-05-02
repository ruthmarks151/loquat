use yew::prelude::*;
use yewdux::prelude::*;
use loquat_common::models::{FanSeries, FanSize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
struct FanStore {
    fan_serieses: HashMap<String, FanSeries>,
    fan_sizes: HashMap<String, FanSize>,

}
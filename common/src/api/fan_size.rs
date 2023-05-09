use crate::models::{FanSeries, FanSize};

pub type IndexResponse = Vec<FanSize<()>>;

pub type GetResponse = FanSize<FanSeries<()>>;

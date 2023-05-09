use crate::models::{FanSeries, FanSize};

pub type IndexResponse = Vec<FanSeries<()>>;

pub type  GetResponse = FanSeries<Vec<FanSize<()>>>;

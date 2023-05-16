use crate::models::{A1Standard2010Report, FanSeries, FanSize};

pub type GetResponse = A1Standard2010Report<FanSize<FanSeries<()>>>;

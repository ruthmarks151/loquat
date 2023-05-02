pub trait TestEvent<Parameters, Determination> {
    fn standard_id(&self) -> &'static str;

    fn determinations(&self) -> Vec<Determination>;
}

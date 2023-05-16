pub mod api;
pub mod features;
pub mod route;
pub mod store;

pub trait SynchronousReducer<Store> {
    fn apply(self, state: &mut Store) -> &mut Store;
}

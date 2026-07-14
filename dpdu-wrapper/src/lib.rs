pub mod api;
pub mod types;
pub mod utils;
pub mod worker;
pub mod handle_manager;
mod event_callback;
pub mod error;

pub use libloading;
use crate::api::PduApi;
use crate::worker::PduAsyncWorker;

#[derive(Debug)]
pub enum AsyncRuntimeTarget<'a> {
    Sync(&'a PduApi),
    Async(&'a PduAsyncWorker)
}

impl<'a> From<&'a PduApi> for AsyncRuntimeTarget<'a> {
    fn from(value: &'a PduApi) -> Self {
        AsyncRuntimeTarget::Sync(value)
    }
}

impl<'a> From<&'a PduAsyncWorker> for AsyncRuntimeTarget<'a> {
    fn from(value: &'a PduAsyncWorker) -> Self {
        AsyncRuntimeTarget::Async(value)
    }
}
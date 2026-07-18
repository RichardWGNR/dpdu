pub mod api;
pub mod error;
mod event_callback;
pub mod handle_manager;
pub mod types;
pub mod utils;
pub mod worker;
mod constants;
mod vendor_specific;

use dpdu_api_types::{PduConstructFn, PduDestructFn};
use crate::api::PduApi;
use crate::worker::PduAsyncWorker;
pub use libloading;

#[derive(Debug)]
pub enum AsyncRuntimeTarget<'a> {
    Api(&'a PduApi),
    Worker(&'a PduAsyncWorker),
}

impl<'a> From<&'a PduApi> for AsyncRuntimeTarget<'a> {
    fn from(value: &'a PduApi) -> Self {
        AsyncRuntimeTarget::Api(value)
    }
}

impl<'a> From<&'a PduAsyncWorker> for AsyncRuntimeTarget<'a> {
    fn from(value: &'a PduAsyncWorker) -> Self {
        AsyncRuntimeTarget::Worker(value)
    }
}
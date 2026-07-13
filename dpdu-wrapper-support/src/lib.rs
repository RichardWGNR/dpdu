use proc_macro::TokenStream;

mod com_param_stack_configurator;
mod worker_rpc;

#[proc_macro]
pub fn declare_worker_rpc(input: TokenStream) -> TokenStream {
    worker_rpc::declare_worker_rpc(input)
}

#[proc_macro]
pub fn impl_configure_from_serde_json_map_for_com_param_stack(input: TokenStream) -> TokenStream {
    com_param_stack_configurator::impl_configure_from_serde_json_map_for_com_param_stack(input)
}

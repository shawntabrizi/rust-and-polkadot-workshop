mod call;
mod runtime;

#[proc_macro_attribute]
pub fn call(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	call::call(attr, item)
}

#[proc_macro_attribute]
pub fn runtime(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	runtime::runtime(attr, item)
}

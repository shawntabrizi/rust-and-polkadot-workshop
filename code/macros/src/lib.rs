mod call;

#[proc_macro_attribute]
pub fn call(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	call::call(attr, item)
}

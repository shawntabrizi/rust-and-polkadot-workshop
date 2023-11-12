mod expand;
mod parse;

#[proc_macro_attribute]
pub fn call(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let mut finished = item.clone();
	let item_mod = syn::parse_macro_input!(item as syn::Item);

	let generated: proc_macro::TokenStream = match parse::CallDef::try_from(item_mod.clone()) {
		Ok(def) => {
			eprintln!("{:?}", def);
			expand::expand_call(def).into()
		},
		Err(e) => e.to_compile_error().into(),
	};

	finished.extend(generated);

	return finished;
}

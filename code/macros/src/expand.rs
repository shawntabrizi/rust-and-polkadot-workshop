use crate::parse::CallDef;
use quote::quote;

/// Expand definition, in particular:
/// * add some bounds and variants to type defined,
/// * create some new types,
/// * impl stuff on them.
pub fn expand_call(def: CallDef) -> proc_macro2::TokenStream {
	let CallDef { module_struct, methods } = def;
	let fn_name = methods.iter().map(|method| &method.name).collect::<Vec<_>>();

	let args_name = methods
		.iter()
		.map(|method| method.args.iter().map(|(name, _)| name.clone()).collect::<Vec<_>>())
		.collect::<Vec<_>>();

	let args_type = methods
		.iter()
		.map(|method| method.args.iter().map(|(_, type_)| type_.clone()).collect::<Vec<_>>())
		.collect::<Vec<_>>();

	let dispatch_impl = quote! {
		#[allow(non_camel_case_types)]
		pub enum Call<T: Config> {
			#(
				#fn_name { #( #args_name: #args_type),* },
			)*
		}

		impl<T: Config> crate::support::Dispatch for #module_struct<T> {
			type Caller = T::AccountId;
			type Call = Call<T>;

			fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> Result<(), &'static str> {
				match call {
					#(
						Call::#fn_name { #( #args_name ),* } => {
							self.#fn_name(
								caller,
								#( #args_name ),*
							)?;
						},
					)*
				}
				Ok(())
			}
		}
	};

	dispatch_impl.into()
}

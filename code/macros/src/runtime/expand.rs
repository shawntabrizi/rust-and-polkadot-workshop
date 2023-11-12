use super::parse::RuntimeDef;
use quote::quote;

/// Expand definition, in particular:
/// * add some bounds and variants to type defined,
/// * create some new types,
/// * impl stuff on them.
pub fn expand_call(def: RuntimeDef) -> proc_macro2::TokenStream {
	let RuntimeDef { runtime_struct, modules } = def;

	let dispatch_impl = quote! {
		// These are all the calls which are exposed to the world.
		// Note that it is just an accumulation of the calls exposed by each module.
		pub enum RuntimeCall {
			#( #modules(#modules::Call<#runtime_struct>) ),*
		}

		impl crate::support::Dispatch for #runtime_struct {
			type Caller = <Runtime as system::Config>::AccountId;
			type Call = RuntimeCall;
			// Dispatch a call on behalf of a caller. Increments the caller's nonce.
			//
			// Dispatch allows us to identify which underlying module call we want to execute.
			// Note that we extract the `caller` from the extrinsic, and use that information
			// to determine who we are executing the call on behalf of.
			fn dispatch(
				&mut self,
				caller: Self::Caller,
				runtime_call: Self::Call,
			) -> Result<(), &'static str> {
				self.system.inc_nonce(&caller);

				match runtime_call {
					#(
						RuntimeCall::#modules(call) => {
							self.#modules.dispatch(caller, call)?;
						}
					),*
				}
				Ok(())
			}
		}
	};

	dispatch_impl.into()
}

use super::parse::RuntimeDef;
use quote::quote;

/// Expand definition, in particular:
/// * add some bounds and variants to type defined,
/// * create some new types,
/// * impl stuff on them.
pub fn expand_call(def: RuntimeDef) -> proc_macro2::TokenStream {
	let RuntimeDef { runtime_struct, modules } = def;

	let module_names = modules.iter().map(|(name, _)| name.clone()).collect::<Vec<_>>();
	let module_types = modules.iter().map(|(_, type_)| type_.clone()).collect::<Vec<_>>();

	let dispatch_impl = quote! {
		impl Runtime {
			// Create a new instance of the main Runtime, by creating a new instance of each module.
			fn new() -> Self {
				Self {
					system: <system::SystemModule::<Self>>::new(),
					#(
						#module_names: <#module_types>::new()
					),*
				}
			}

			// Execute a block of extrinsics. Increments the block number.
			fn execute_block(&mut self, block: types::Block) -> Result<(), &'static str> {
				self.system.inc_block_number();
				if block.header.block_number != self.system.block_number() {
					return Err(&"block number does not match what is expected")
				}
				for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
					let _res = self.dispatch(caller, call).map_err(|e| {
						eprintln!(
							"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
							block.header.block_number, i, e
						)
					});
				}
				Ok(())
			}
		}

		// These are all the calls which are exposed to the world.
		// Note that it is just an accumulation of the calls exposed by each module.
		#[allow(non_camel_case_types)]
		pub enum RuntimeCall {
			#( #module_names(#module_names::Call<#runtime_struct>) ),*
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
						RuntimeCall::#module_names(call) => {
							self.#module_names.dispatch(caller, call)?;
						}
					),*
				}
				Ok(())
			}
		}
	};

	dispatch_impl.into()
}

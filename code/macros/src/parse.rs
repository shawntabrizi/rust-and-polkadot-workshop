use quote::ToTokens;
use syn::spanned::Spanned;

mod keyword {
	syn::custom_keyword!(pallet);
	syn::custom_keyword!(call);
	syn::custom_keyword!(T);
	syn::custom_keyword!(AccountId);
}

#[derive(Debug)]
pub struct CallDef {
	pub module_struct: syn::Ident,
	pub methods: Vec<CallVariantDef>,
}

#[derive(Debug)]
pub struct CallVariantDef {
	/// Function name.
	pub name: syn::Ident,
	/// Information on args: `(name, type)`
	pub args: Vec<(syn::Ident, Box<syn::Type>)>,
}

impl CallDef {
	pub fn try_from(item: syn::Item) -> syn::Result<Self> {
		let item_impl = if let syn::Item::Impl(item) = item {
			item
		} else {
			return Err(syn::Error::new(item.span(), "Invalid pallet::call, expected item impl"))
		};

		let module_struct = match &*item_impl.self_ty {
			syn::Type::Path(tp) => tp.path.segments.first().unwrap().ident.clone(),
			_ => panic!("not supported tokens"),
		};

		let mut methods = vec![];
		for item in item_impl.items {
			if let syn::ImplItem::Fn(method) = item {
				let mut args = vec![];

				// first argument should be `self`
				match method.sig.inputs.first() {
					Some(syn::FnArg::Receiver(_)) => {},
					_ => {
						let msg = "Invalid call, first argument must self";
						return Err(syn::Error::new(method.sig.span(), msg))
					},
				}

				// Extract the first argument's name (if any) from the function
				match method.sig.inputs.iter().skip(1).next() {
					Some(syn::FnArg::Typed(arg)) => {
						check_caller_arg(arg)?;
					},
					_ => {
						let msg = "Invalid call, second argument should be `caller: T::AccountId`";
						return Err(syn::Error::new(method.sig.span(), msg))
					},
				}

				// skipping 2 for `self` and `caller`
				for arg in method.sig.inputs.iter().skip(2) {
					let arg = if let syn::FnArg::Typed(arg) = arg {
						arg
					} else {
						unreachable!("Only first argument can be receiver");
					};

					let arg_ident = if let syn::Pat::Ident(pat) = &*arg.pat {
						pat.ident.clone()
					} else {
						let msg = "Invalid pallet::call, argument must be ident";
						return Err(syn::Error::new(arg.pat.span(), msg))
					};

					args.push((arg_ident, arg.ty.clone()));
				}

				methods.push(CallVariantDef { name: method.sig.ident.clone(), args });
			}
		}

		Ok(Self { module_struct, methods })
	}
}

/// Check caller arg is exactly: `caller: T::AccountId`
pub fn check_caller_arg(arg: &syn::PatType) -> syn::Result<()> {
	pub struct CheckDispatchableFirstArg;
	impl syn::parse::Parse for CheckDispatchableFirstArg {
		fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
			input.parse::<keyword::T>()?;
			input.parse::<syn::Token![::]>()?;
			input.parse::<keyword::AccountId>()?;
			Ok(Self)
		}
	}

	let ty = &arg.ty;
	syn::parse2::<CheckDispatchableFirstArg>(ty.to_token_stream()).map_err(|e| {
		let msg = "Invalid type for second parameter: expected `caller: T::AccountId`";
		let mut err = syn::Error::new(ty.span(), msg);
		err.combine(e);
		err
	})?;

	if let syn::Pat::Ident(ident) = &*arg.pat {
		if &ident.ident != "caller" && &ident.ident != "_caller" {
			let msg = "Invalid name for second parameter: expected `caller: T::AccountId`";
			return Err(syn::Error::new(ty.span(), msg))
		}
	}

	Ok(())
}

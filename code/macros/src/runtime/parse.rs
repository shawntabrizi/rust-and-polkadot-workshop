use syn::spanned::Spanned;

#[derive(Debug)]
pub struct RuntimeDef {
	pub runtime_struct: syn::Ident,
	pub modules: Vec<(syn::Ident, syn::Type)>,
}

impl RuntimeDef {
	pub fn try_from(item: syn::Item) -> syn::Result<Self> {
		let item_struct = if let syn::Item::Struct(item) = item {
			item
		} else {
			return Err(syn::Error::new(item.span(), "Invalid runtime, expected item struct"))
		};

		check_system(&item_struct)?;

		let runtime_struct = item_struct.ident;

		let mut modules = vec![];
		// skip system
		for field in item_struct.fields.into_iter().skip(1) {
			if let Some(ident) = field.ident {
				modules.push((ident, field.ty))
			}
		}

		Ok(Self { runtime_struct, modules })
	}
}

fn check_system(item_struct: &syn::ItemStruct) -> syn::Result<()> {
	// Extract the name of the first field
	let first_field_name = if let Some(first_field) = item_struct.fields.iter().next() {
		if let Some(field_name) = &first_field.ident {
			field_name.to_string()
		} else {
			let msg = "first field is expected to be system";
			return Err(syn::Error::new(item_struct.span(), msg))
		}
	} else {
		let msg = "runtime struct is expected to have fields";
		return Err(syn::Error::new(item_struct.span(), msg))
	};

	// Check if the first field is named "system"
	if first_field_name != "system" {
		let msg = "first field is expected to be system";
		return Err(syn::Error::new(item_struct.span(), msg))
	}

	Ok(())
}

use std::collections::BTreeMap;

// This is the System Module.
// It handles low level meta information needed for your blockchain.
#[derive(Default, Debug)]
pub struct SystemModule {
	block_number: u32,
	nonce: BTreeMap<&'static str, u32>,
}

// The System Module is a low level system which is not really meant to be exposed to users.
// Instead, these functions are used by your low level blockchain systems.
impl SystemModule {
	// Create a new instance of the System Module.
	pub fn new() -> Self {
		Self::default()
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}

	// Increment the nonce of a user. This helps us keep track of how many transactions each user
	// has made.
	pub fn inc_nonce(&mut self, who: &'static str) {
		let nonce = self.nonce.get(who).unwrap_or(&0);
		let new_nonce = nonce + 1;
		self.nonce.insert(who, new_nonce);
	}
}

#[test]
fn init_system() {
	let mut system = SystemModule::new();
	system.inc_block_number();
	system.inc_nonce(&"alice");

	assert_eq!(system.block_number, 1);
	assert_eq!(system.nonce.get(&"alice"), Some(&1));
	assert_eq!(system.nonce.get(&"bob"), None);
}

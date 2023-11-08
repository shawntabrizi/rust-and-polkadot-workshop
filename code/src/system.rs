use core::{
	fmt::Debug,
	ops::{Add, AddAssign},
};
use num::traits::{One, Zero};
use std::collections::BTreeMap;

/// The configuration trait for the System Module.
/// This controls the common types used throughout our state machine.
pub trait Config {
	/// A type which can identify an account in our state machine.
	/// On a real blockchain, you would want this to be a cryptographic public key.
	type AccountId: Debug + Default + Ord + Copy;
	/// A type which can be used to represent the current block number.
	/// Usually a basic unsigned integer.
	type BlockNumber: Debug + Default + One + Zero + AddAssign;
	/// A type which can be used to keep track of the number of transactions from each account.
	/// Usually a basic unsigned integer.
	type Nonce: Debug + Default + One + Zero + Add + Copy;
}

/// This is the System Module.
/// It handles low level state needed for your blockchain.
#[derive(Default, Debug)]
pub struct SystemModule<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

/// The System Module is a low level system which is not really meant to be exposed to the outside
/// world. Instead, these functions are used by your low level blockchain systems.
impl<T: Config> SystemModule<T> {
	/// Create a new instance of the System Module.
	pub fn new() -> Self {
		Self { block_number: Default::default(), nonce: Default::default() }
	}

	/// This function can be used to increment the block number.
	/// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	/// Increment the nonce of an account. This helps us keep track of how many transactions each
	/// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		let new_nonce = nonce + T::Nonce::one();
		self.nonce.insert(*who, new_nonce);
	}
}

#[cfg(test)]
mod test {

	struct TestConfig;
	impl super::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		let mut system = super::SystemModule::<TestConfig>::new();
		assert_eq!(system.block_number, 0);

		system.inc_block_number();
		system.inc_nonce(&"alice");

		assert_eq!(system.block_number, 1);
		assert_eq!(system.nonce.get(&"alice"), Some(&1));
		assert_eq!(system.nonce.get(&"bob"), None);
	}
}

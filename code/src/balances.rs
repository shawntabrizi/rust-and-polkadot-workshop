use crate::support::DispatchResult;
use core::fmt::Debug;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

/// The configuration trait for the Balances Module.
/// Contains the basic types needed for handling balances.
pub trait Config: crate::system::Config {
	/// A type which can represent the balance of an account.
	/// Usually this is a large unsigned integer.
	type Balance: Zero + CheckedAdd + CheckedSub + Copy + Debug;
}

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct BalancesModule<T: Config> {
	/// A map from account to the balance they have.
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> BalancesModule<T> {
	/// Create a new instance of the Balances Module.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	/// Set the balance of an account `who` to some `amount`.
	pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) {
		self.balances.insert(who, amount);
	}

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
	pub fn balance(&self, who: T::AccountId) -> T::Balance {
		*self.balances.get(&who).unwrap_or(&T::Balance::zero())
	}
}

#[macros::call]
impl<T: Config> BalancesModule<T> {
	/// Transfer `amount` from one another to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		caller: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> DispatchResult {
		let caller_balance = self.balance(caller);
		let to_balance = self.balance(to);

		let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

		self.balances.insert(caller, new_caller_balance);
		self.balances.insert(to, new_to_balance);

		Ok(())
	}

	pub fn lala(&mut self, _caller: T::AccountId) -> DispatchResult {
		Ok(())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;
	impl super::Config for TestConfig {
		type Balance = u128;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_balance() {
		let mut balances = super::BalancesModule::<TestConfig>::new();

		assert_eq!(balances.balance(&"alice"), 0);
		balances.set_balance(&"alice", 100);
		assert_eq!(balances.balance(&"alice"), 100);
		assert_eq!(balances.balance(&"bob"), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::BalancesModule::<TestConfig>::new();

		assert_eq!(balances.transfer(&"alice", &"bob", 51), Err("Not enough funds."));

		balances.set_balance(&"alice", 100);
		assert_eq!(balances.transfer(&"alice", &"bob", 51), Ok(()));
		assert_eq!(balances.balance(&"alice"), 49);
		assert_eq!(balances.balance(&"bob"), 51);

		assert_eq!(balances.transfer(&"alice", &"bob", 51), Err("Not enough funds."));
	}
}

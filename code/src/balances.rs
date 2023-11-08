use core::fmt::Debug;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

/// The configuration trait for the Balances Module.
/// Contains the basic types needed for handling balances.
pub trait Config: super::system::Config {
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

	/// Transfer `amount` from one another to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		from: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> Result<(), &'static str> {
		let from_balance = self.balance(from);
		let to_balance = self.balance(to);

		let new_from_balance = from_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

		self.balances.insert(from, new_from_balance);
		self.balances.insert(to, new_to_balance);

		Ok(())
	}
}

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum BalancesCall<T: Config> {
	Transfer { to: T::AccountId, amount: T::Balance },
}

/// Implementation of the dispatch logic, mapping from `BalancesCall` to the appropriate underlying
/// function we want to execute.
impl<T: Config> crate::support::Dispatch for BalancesModule<T> {
	type Caller = T::AccountId;
	type Call = BalancesCall<T>;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> Result<(), &'static str> {
		match call {
			BalancesCall::Transfer { to, amount } => {
				self.transfer(caller, to, amount)?;
			},
		}
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

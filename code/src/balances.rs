use core::fmt::Debug;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: super::system::Config {
	type Balance: Zero + CheckedAdd + CheckedSub + Copy + Debug;
}

// This is the Balances Module.
// It is a simple module which keeps track of how much balance each user has in this state machine.
#[derive(Debug)]
pub struct BalancesModule<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> BalancesModule<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) {
		self.balances.insert(who, amount);
	}

	pub fn balance(&self, who: T::AccountId) -> T::Balance {
		*self.balances.get(&who).unwrap_or(&T::Balance::zero())
	}

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

impl<T: Config> crate::Dispatch<T::AccountId, BalancesCall<T>> for BalancesModule<T> {
	fn dispatch(
		&mut self,
		caller: T::AccountId,
		call: BalancesCall<T>,
	) -> Result<(), &'static str> {
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

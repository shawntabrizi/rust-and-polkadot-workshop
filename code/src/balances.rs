use core::fmt::Debug;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config {
	type Balance: Zero + CheckedAdd + CheckedSub + Copy + Debug;
}

// This is the Balances Module.
// It is a simple module which keeps track of how much balance each user has in this state machine.
#[derive(Debug)]
pub struct BalancesModule<T: Config> {
	balances: BTreeMap<&'static str, T::Balance>,
}

impl<T: Config> BalancesModule<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &'static str, amount: T::Balance) {
		self.balances.insert(who, amount);
	}

	pub fn balance(&self, who: &'static str) -> T::Balance {
		*self.balances.get(&who).unwrap_or(&T::Balance::zero())
	}

	pub fn transfer(
		&mut self,
		from: &'static str,
		to: &'static str,
		amount: T::Balance,
	) -> Result<(), &'static str> {
		let from_balance = self.balance(&from);
		let to_balance = self.balance(&to);

		let new_from_balance = from_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

		self.balances.insert(from, new_from_balance);
		self.balances.insert(to, new_to_balance);

		Ok(())
	}
}

// A public enum which describes the calls we want to expose
pub enum BalancesCall<T: Config> {
	Transfer { to: &'static str, amount: T::Balance },
}

#[cfg(test)]
mod test {
	struct TestConfg;
	impl super::Config for TestConfg {
		type Balance = u128;
	}

	#[test]
	fn init_balance() {
		let mut balances = super::BalancesModule::<TestConfg>::new();

		assert_eq!(balances.balance(&"alice"), 0);
		balances.set_balance(&"alice", 100);
		assert_eq!(balances.balance(&"alice"), 100);
		assert_eq!(balances.balance(&"bob"), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::BalancesModule::<TestConfg>::new();

		assert_eq!(balances.transfer(&"alice", &"bob", 51), Err("Not enough funds."));

		balances.set_balance(&"alice", 100);
		assert_eq!(balances.transfer(&"alice", &"bob", 51), Ok(()));
		assert_eq!(balances.balance(&"alice"), 49);
		assert_eq!(balances.balance(&"bob"), 51);

		assert_eq!(balances.transfer(&"alice", &"bob", 51), Err("Not enough funds."));
	}
}

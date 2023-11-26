use std::collections::BTreeMap;

/*
	TODO: Define the common types used in this pallet:
		- `AccountID`
		- `Balance`

	Then update this pallet to use these common types.

	HINT: Remember that our `Result` does not return `AccountID`, so keep this `&'static str`.
*/

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct Pallet {
	// A simple storage mapping from accounts (`&'static str`) to their balances (`u128`).
	balances: BTreeMap<&'static str, u128>,
}

impl Pallet {
	/// Create a new instance of the balances module.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	/// Set the balance of an account `who` to some `amount`.
	pub fn set_balance(&mut self, who: &'static str, amount: u128) {
		self.balances.insert(who, amount);
	}

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
	pub fn balance(&self, who: &'static str) -> u128 {
		*self.balances.get(&who).unwrap_or(&0)
	}

	/// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		caller: &'static str,
		to: &'static str,
		amount: u128,
	) -> Result<(), &'static str> {
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

		self.balances.insert(caller, new_caller_balance);
		self.balances.insert(to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::new();

		assert_eq!(balances.balance(&"alice"), 0);
		balances.set_balance(&"alice", 100);
		assert_eq!(balances.balance(&"alice"), 100);
		assert_eq!(balances.balance(&"bob"), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::new();

		assert_eq!(balances.transfer(&"alice", &"bob", 51), Err("Not enough funds."));

		balances.set_balance(&"alice", 100);
		assert_eq!(balances.transfer(&"alice", &"bob", 51), Ok(()));
		assert_eq!(balances.balance(&"alice"), 49);
		assert_eq!(balances.balance(&"bob"), 51);

		assert_eq!(balances.transfer(&"alice", &"bob", 51), Err("Not enough funds."));
	}
}

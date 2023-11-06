use std::collections::BTreeMap;

#[derive(Debug)]
pub struct BalancesModule {
	balances: BTreeMap<&'static str, u128>,
}

pub enum BalancesCall {
	Transfer { to: &'static str, amount: u128 },
}

impl BalancesModule {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &'static str, amount: u128) {
		self.balances.insert(who, amount);
	}

	pub fn balance(&self, who: &'static str) -> u128 {
		*self.balances.get(&who).unwrap_or(&0)
	}

	pub fn transfer(
		&mut self,
		from: &'static str,
		to: &'static str,
		amount: u128,
	) -> Result<(), &'static str> {
		let from_balance = self.balance(&from);
		let to_balance = self.balance(&to);

		let new_from_balance = from_balance.checked_sub(amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

		self.balances.insert(from, new_from_balance);
		self.balances.insert(to, new_to_balance);

		Ok(())
	}
}

#[test]
fn init_balance() {
	let mut balances = BalancesModule::new();

	assert_eq!(balances.balance(&"alice"), 0);
	balances.set_balance(&"alice", 100);
	assert_eq!(balances.balance(&"alice"), 100);
	assert_eq!(balances.balance(&"bob"), 0);
}

#[test]
fn transfer_balance() {
	let mut balances = BalancesModule::new();

	assert_eq!(balances.transfer(&"alice", &"bob", 51), Err("Not enough funds."));

	balances.set_balance(&"alice", 100);
	assert_eq!(balances.transfer(&"alice", &"bob", 51), Ok(()));
	assert_eq!(balances.balance(&"alice"), 49);
	assert_eq!(balances.balance(&"bob"), 51);

	assert_eq!(balances.transfer(&"alice", &"bob", 51), Err("Not enough funds."));
}

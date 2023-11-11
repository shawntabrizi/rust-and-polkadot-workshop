use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct POEModule<T: Config> {
	claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> POEModule<T> {
	/// Create a new instance of the POE Module.
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(&claim)
	}

	pub fn create_claim(&mut self, claimer: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err(&"this content is already claimed");
		}
		self.claims.insert(claim, claimer);
		Ok(())
	}

	pub fn revoke_claim(&mut self, claimer: T::AccountId, claim: T::Content) -> DispatchResult {
		let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
		if claimer != *owner {
			return Err(&"this content is owned by someone else");
		}
		self.claims.remove(&claim);
		Ok(())
	}
}

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum POECall<T: Config> {
	CreateClaim { claim: T::Content },
	RevokeClaim { claim: T::Content },
}

/// Implementation of the dispatch logic, mapping from `POECall` to the appropriate underlying
/// function we want to execute.
impl<T: Config> crate::support::Dispatch for POEModule<T> {
	type Caller = T::AccountId;
	type Call = POECall<T>;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> Result<(), &'static str> {
		match call {
			POECall::CreateClaim { claim } => {
				self.create_claim(caller, claim)?;
			},
			POECall::RevokeClaim { claim } => {
				self.revoke_claim(caller, claim)?;
			},
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;
	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut poe = super::POEModule::<TestConfig>::new();
		assert_eq!(poe.get_claim(&"Hello, world!"), None);
		assert_eq!(poe.create_claim(&"alice", &"Hello, world!"), Ok(()));
		assert_eq!(poe.get_claim(&"Hello, world!"), Some(&"alice"));
		assert_eq!(
			poe.create_claim(&"bob", &"Hello, world!"),
			Err("this content is already claimed")
		);
		assert_eq!(poe.revoke_claim(&"alice", &"Hello, world!"), Ok(()));
		assert_eq!(poe.create_claim(&"bob", &"Hello, world!"), Ok(()));
	}
}

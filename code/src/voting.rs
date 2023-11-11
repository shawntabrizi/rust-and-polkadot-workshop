use crate::support::DispatchResult;
use core::{fmt::Debug, ops::Add};
use num::traits::{CheckedAdd, One, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	const VOTES_TO_CLOSE: u16;
	type ProposalIndex: Debug + Default + One + Zero + Add + CheckedAdd + Copy + Ord;
	type ProposalInfo: Debug;
}

#[derive(Debug)]
struct Proposal<T: Config> {
	creator: T::AccountId,
	info: T::ProposalInfo,
	votes: BTreeMap<T::AccountId, bool>,
	outcome: Option<bool>,
}

/// This is the Voting Module.
/// It allows accounts to generate new proposals, and then all accounts can either
/// for for or against that proposal.
#[derive(Debug)]
pub struct VotingModule<T: Config> {
	/// The next proposal index.
	proposal_index: T::ProposalIndex,
	/// Storage for all current and past proposals.
	proposals: BTreeMap<T::ProposalIndex, Proposal<T>>,
}

impl<T: Config> VotingModule<T> {
	/// Create a new instance of the Balances Module.
	pub fn new() -> Self {
		Self { proposal_index: T::ProposalIndex::zero(), proposals: BTreeMap::new() }
	}

	pub fn create_proposal(
		&mut self,
		creator: T::AccountId,
		info: T::ProposalInfo,
	) -> DispatchResult {
		let index = self.proposal_index;
		let next_index =
			index.checked_add(&T::ProposalIndex::one()).ok_or("proposal index overflow")?;
		let proposal: Proposal<T> =
			Proposal { creator, info, votes: BTreeMap::new(), outcome: None };

		self.proposals.insert(index, proposal);
		self.proposal_index = next_index;
		Ok(())
	}

	pub fn vote(
		&mut self,
		who: T::AccountId,
		proposal_id: T::ProposalIndex,
		aye: bool,
	) -> DispatchResult {
		let proposal = self.proposals.get_mut(&proposal_id).ok_or("proposal doesn't exist")?;
		proposal.votes.insert(who, aye);
		Ok(())
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn test_vote() {
		#[derive(Debug)]
		struct TestConfig;
		impl crate::system::Config for TestConfig {
			type AccountId = &'static str;
			type BlockNumber = u32;
			type Nonce = u32;
		}
		impl super::Config for TestConfig {
			const VOTES_TO_CLOSE: u16 = 10u16;
			type ProposalIndex = u64;
			type ProposalInfo = &'static str;
		}

		let mut voting = super::VotingModule::<TestConfig>::new();

		assert_eq!(voting.vote(&"alice", 0, true), Err("proposal doesn't exist"));
		assert_eq!(voting.create_proposal(&"alice", &"Let's save the world."), Ok(()));
		assert_eq!(voting.vote(&"alice", 0, true), Ok(()));
		assert_eq!(voting.proposals.get(&0).unwrap().votes.len(), 1);
	}
}

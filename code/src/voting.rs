use crate::support::DispatchResult;
use core::{fmt::Debug, ops::Add};
use num::traits::{CheckedAdd, One, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	type ProposalIndex: Debug + Default + One + Zero + Add + CheckedAdd + Copy;
	type ProposalInfo: Debug;
}

#[derive(Debug)]
struct Vote<T: Config> {
	who: T::AccountId,
	aye: bool,
}

#[derive(Debug)]
struct Proposal<T: Config> {
	creator: T::AccountId,
	info: T::ProposalInfo,
	votes: Vec<Vote<T>>,
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
		let proposal = Proposal { creator, info, votes: Vec::<Vote<T>>::new(), outcome: None };

		self.proposal_index = next_index;
		Ok(())
	}
}

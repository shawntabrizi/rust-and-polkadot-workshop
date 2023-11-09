mod balances;
mod support;
mod system;
mod voting;

use crate::support::Dispatch;

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
	pub type AccountId = &'static str;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Balance = u128;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Block = crate::support::Block<BlockNumber, Extrinsic>;
}

// This is our main Runtime.
// It accumulates all of the different modules we want to use,
// functions implemented on the Runtime allow us to access those modules and execute blocks of
// transactions.
#[derive(Debug)]
pub struct Runtime {
	system: system::SystemModule<Self>,
	balances: balances::BalancesModule<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each module.
	fn new() -> Self {
		Self { system: system::SystemModule::new(), balances: balances::BalancesModule::new() }
	}

	// Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> Result<(), &'static str> {
		self.system.inc_block_number();
		if block.header.block_number != self.system.block_number() {
			return Err(&"block number does not match what is expected")
		}
		for support::Extrinsic { caller, call } in block.extrinsics {
			self.dispatch(caller, call)?;
		}
		Ok(())
	}
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
	Balances(balances::BalancesCall<Runtime>),
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> Result<(), &'static str> {
		self.system.inc_nonce(&caller);

		match runtime_call {
			RuntimeCall::Balances(call) => {
				self.balances.dispatch(caller, call)?;
			},
		}
		Ok(())
	}
}

// The main entry point for our simple state machine.
fn main() {
	// Create a new instance of the Runtime.
	// It will instantiate with it all the modules it uses.
	let mut runtime = Runtime::new();

	// Initialize the system with some initial balance.
	runtime.balances.set_balance(&"alice", 100);

	// Here are the extrinsics in our block.
	// You can add or remove these based on the modules and calls you have set up.
	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: &"alice",
				call: RuntimeCall::Balances(balances::BalancesCall::Transfer {
					to: &"bob",
					amount: 20,
				}),
			},
			support::Extrinsic {
				caller: &"alice",
				call: RuntimeCall::Balances(balances::BalancesCall::Transfer {
					to: &"charlie",
					amount: 20,
				}),
			},
		],
	};

	// Execute the extrinsics which make up our block.
	// If there are any errors, our system panics, since we should not execute invalid blocks.
	runtime.execute_block(block_1).expect("invalid block");

	// Simply print the debug format of our runtime state.
	println!("{:#?}", runtime);
}

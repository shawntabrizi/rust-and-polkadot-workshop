mod balances;
mod system;

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
enum RuntimeCall {
	Balances(balances::BalancesCall<Runtime>),
}

// This is an "extrinsic": literally an object from outside of the blockchain.
// It tells us who is making the call, and which call they are making.
struct Extrinsic {
	caller: &'static str,
	call: RuntimeCall,
}

trait RuntimeConfig: balances::Config {}

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
	type AccountId = &'static str;
	type BlockNumber = u32;
	type Nonce = u32;
}

impl balances::Config for Runtime {
	type Balance = u128;
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each module.
	fn new() -> Self {
		Self { system: system::SystemModule::new(), balances: balances::BalancesModule::new() }
	}

	// Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, extrinsics: Vec<Extrinsic>) -> Result<(), &'static str> {
		self.system.inc_block_number();
		for extrinsic in extrinsics {
			self.dispatch(extrinsic)?;
		}
		Ok(())
	}

	// Dispatch a specific extrinsic. Increments the user's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(&mut self, extrinsic: Extrinsic) -> Result<(), &'static str> {
		let Extrinsic { call, caller } = extrinsic;
		self.system.inc_nonce(&caller);

		match call {
			RuntimeCall::Balances(balances::BalancesCall::Transfer { to, amount }) => {
				self.balances.transfer(caller, to, amount)?;
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
	let extrinsics = vec![
		Extrinsic {
			caller: &"alice",
			call: RuntimeCall::Balances(balances::BalancesCall::Transfer {
				to: &"bob",
				amount: 20,
			}),
		},
		Extrinsic {
			caller: &"alice",
			call: RuntimeCall::Balances(balances::BalancesCall::Transfer {
				to: &"charlie",
				amount: 20,
			}),
		},
	];

	// Execute the extrinsics which make up our block.
	// If there are any errors, our system panics, since we should not execute invalid blocks.
	runtime.execute_block(extrinsics).expect("invalid block");

	// Simply print the debug format of our runtime state.
	println!("{:#?}", runtime);
}

mod balances;
mod system;

enum RuntimeCall {
	Balances(balances::BalancesCall),
}

struct Extrinsic {
	caller: &'static str,
	call: RuntimeCall,
}

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

// This is our main Runtime.
// It accumulates all of the different modules we want to use,
// functions implemented on the Runtime allow us to access those modules and execute blocks of
// transactions.
#[derive(Debug)]
struct Runtime {
	pub system: system::SystemModule,
	pub balances: balances::BalancesModule,
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
		self.system.inc_nonce(caller);

		match call {
			RuntimeCall::Balances(balances::BalancesCall::Transfer { to, amount }) => {
				self.balances.transfer(caller, to, amount)?;
			},
		}

		Ok(())
	}
}

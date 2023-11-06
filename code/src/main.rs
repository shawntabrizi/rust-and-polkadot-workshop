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
    println!("Hello, world!");

    let mut runtime = Runtime::new();

    runtime.balances.set_balance(&"alice", 100);

    runtime
        .execute_block(vec![
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
        ])
        .expect("invalid block");

    println!("{:#?}", runtime);
}

#[derive(Debug)]
struct Runtime {
    pub system: system::SystemModule,
    pub balances: balances::BalancesModule,
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::SystemModule::new(),
            balances: balances::BalancesModule::new(),
        }
    }

    fn execute_block(&mut self, extrinsics: Vec<Extrinsic>) -> Result<(), &'static str> {
        self.system.inc_block_number();
        for extrinsic in extrinsics {
            self.dispatch(extrinsic)?;
        }
        Ok(())
    }

    fn dispatch(&mut self, extrinsic: Extrinsic) -> Result<(), &'static str> {
        let Extrinsic { call, caller } = extrinsic;
        self.system.inc_nonce(caller);

        match call {
            RuntimeCall::Balances(balances::BalancesCall::Transfer { to, amount }) => {
                self.balances.transfer(caller, to, amount)?;
            }
        }

        Ok(())
    }
}

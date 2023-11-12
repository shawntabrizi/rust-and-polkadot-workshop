#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod balances {
    use core::fmt::Debug;
    use num::traits::{CheckedAdd, CheckedSub, Zero};
    use std::collections::BTreeMap;
    use crate::support::DispatchResult;
    /// The configuration trait for the Balances Module.
    /// Contains the basic types needed for handling balances.
    pub trait Config: crate::system::Config {
        /// A type which can represent the balance of an account.
        /// Usually this is a large unsigned integer.
        type Balance: Zero + CheckedAdd + CheckedSub + Copy + Debug;
    }
    /// This is the Balances Module.
    /// It is a simple module which keeps track of how much balance each account has in this state
    /// machine.
    pub struct BalancesModule<T: Config> {
        /// A map from account to the balance they have.
        balances: BTreeMap<T::AccountId, T::Balance>,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug + Config> ::core::fmt::Debug for BalancesModule<T>
    where
        T::AccountId: ::core::fmt::Debug,
        T::Balance: ::core::fmt::Debug,
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "BalancesModule",
                "balances",
                &&self.balances,
            )
        }
    }
    impl<T: Config> BalancesModule<T> {
        /// Create a new instance of the Balances Module.
        pub fn new() -> Self {
            Self {
                balances: BTreeMap::new(),
            }
        }
        /// Set the balance of an account `who` to some `amount`.
        pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) {
            self.balances.insert(who, amount);
        }
        /// Get the balance of an account `who`.
        /// If the account has no stored balance, we return zero.
        pub fn balance(&self, who: T::AccountId) -> T::Balance {
            *self.balances.get(&who).unwrap_or(&T::Balance::zero())
        }
    }
    impl<T: Config> BalancesModule<T> {
        /// Transfer `amount` from one another to another.
        /// This function verifies that `from` has at least `amount` balance to transfer,
        /// and that no mathematical overflows occur.
        pub fn transfer(
            &mut self,
            caller: T::AccountId,
            to: T::AccountId,
            amount: T::Balance,
        ) -> DispatchResult {
            let caller_balance = self.balance(caller);
            let to_balance = self.balance(to);
            let new_caller_balance = caller_balance
                .checked_sub(&amount)
                .ok_or("Not enough funds.")?;
            let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;
            self.balances.insert(caller, new_caller_balance);
            self.balances.insert(to, new_to_balance);
            Ok(())
        }
        pub fn lala(&mut self, caller: T::AccountId) -> DispatchResult {
            Ok(())
        }
    }
    pub enum Call<T: Config> {
        transfer {
            to: T::AccountId,
            amount: T::Balance,
        },
        lala {},
    }
    impl<T: Config> crate::support::Dispatch for BalancesModule<T> {
        type Caller = T::AccountId;
        type Call = Call<T>;
        fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> Result<(), &'static str> {
            match call {
                Call::transfer { to, amount } => {
                    self.transfer(caller, to, amount)?;
                }
                Call::lala { to, amount } => {
                    self.lala(caller)?;
                }
            }
            Ok(())
        }
    }
}
mod proof_of_existence {
    use crate::support::DispatchResult;
    use core::fmt::Debug;
    use std::collections::BTreeMap;
    pub trait Config: crate::system::Config {
        type Content: Debug + Ord;
    }
    pub struct POEModule<T: Config> {
        claims: BTreeMap<T::Content, T::AccountId>,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug + Config> ::core::fmt::Debug for POEModule<T>
    where
        T::Content: ::core::fmt::Debug,
        T::AccountId: ::core::fmt::Debug,
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "POEModule",
                "claims",
                &&self.claims,
            )
        }
    }
    impl<T: Config> POEModule<T> {
        /// Create a new instance of the POE Module.
        pub fn new() -> Self {
            Self {
                claims: BTreeMap::new(),
            }
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
                }
                POECall::RevokeClaim { claim } => {
                    self.revoke_claim(caller, claim)?;
                }
            }
            Ok(())
        }
    }
}
mod support {
    /// The most primitive representation of a Blockchain block.
    pub struct Block<BlockNumber, Extrinsic> {
        /// The block header contains metadata about the block.
        pub header: Header<BlockNumber>,
        /// The extrinsics represent the state transitions to be executed in this block.
        pub extrinsics: Vec<Extrinsic>,
    }
    /// We are using an extremely simplified header which only contains the current block number.
    /// On a real blockchain, you would expect to also find:
    /// - parent block hash
    /// - state root
    /// - extrinsics root
    /// - etc...
    pub struct Header<BlockNumber> {
        pub block_number: BlockNumber,
    }
    /// This is an "extrinsic": literally an external message from outside of the blockchain.
    /// This simplified version of an extrinsic tells us who is making the call, and which call they are
    /// making.
    pub struct Extrinsic<Caller, Call> {
        pub caller: Caller,
        pub call: Call,
    }
    pub type DispatchResult = Result<(), &'static str>;
    /// A trait which allows us to dispatch an incoming extrinsic to the appropriate state transition
    /// function call.
    pub trait Dispatch {
        /// The type used to identify the caller of the function.
        type Caller;
        /// The state transition function call the caller is trying to access.
        type Call;
        /// This function takes a `caller` and the `call` they want to make, and returns a `Result`
        /// based on the outcome of that function call.
        fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
    }
}
mod system {
    use core::{
        fmt::Debug,
        ops::{Add, AddAssign},
    };
    use num::traits::{One, Zero};
    use std::collections::BTreeMap;
    /// The configuration trait for the System Module.
    /// This controls the common types used throughout our state machine.
    pub trait Config {
        /// A type which can identify an account in our state machine.
        /// On a real blockchain, you would want this to be a cryptographic public key.
        type AccountId: Debug + Default + Ord + Copy;
        /// A type which can be used to represent the current block number.
        /// Usually a basic unsigned integer.
        type BlockNumber: Debug + Default + One + Zero + AddAssign + Copy;
        /// A type which can be used to keep track of the number of transactions from each account.
        /// Usually a basic unsigned integer.
        type Nonce: Debug + Default + One + Zero + Add + Copy;
    }
    /// This is the System Module.
    /// It handles low level state needed for your blockchain.
    pub struct SystemModule<T: Config> {
        /// The current block number.
        block_number: T::BlockNumber,
        /// A map from an account to their nonce.
        nonce: BTreeMap<T::AccountId, T::Nonce>,
    }
    #[automatically_derived]
    impl<T: ::core::default::Default + Config> ::core::default::Default for SystemModule<T>
    where
        T::BlockNumber: ::core::default::Default,
        T::AccountId: ::core::default::Default,
        T::Nonce: ::core::default::Default,
    {
        #[inline]
        fn default() -> SystemModule<T> {
            SystemModule {
                block_number: ::core::default::Default::default(),
                nonce: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug + Config> ::core::fmt::Debug for SystemModule<T>
    where
        T::BlockNumber: ::core::fmt::Debug,
        T::AccountId: ::core::fmt::Debug,
        T::Nonce: ::core::fmt::Debug,
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "SystemModule",
                "block_number",
                &self.block_number,
                "nonce",
                &&self.nonce,
            )
        }
    }
    /// The System Module is a low level system which is not really meant to be exposed to the outside
    /// world. Instead, these functions are used by your low level blockchain systems.
    impl<T: Config> SystemModule<T> {
        /// Create a new instance of the System Module.
        pub fn new() -> Self {
            Self {
                block_number: Default::default(),
                nonce: Default::default(),
            }
        }
        /// Get the current block number.
        pub fn block_number(&self) -> T::BlockNumber {
            self.block_number
        }
        /// This function can be used to increment the block number.
        /// Increases the block number by one.
        pub fn inc_block_number(&mut self) {
            self.block_number += T::BlockNumber::one();
        }
        /// Increment the nonce of an account. This helps us keep track of how many transactions each
        /// account has made.
        pub fn inc_nonce(&mut self, who: &T::AccountId) {
            let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
            let new_nonce = nonce + T::Nonce::one();
            self.nonce.insert(*who, new_nonce);
        }
    }
}
use crate::support::Dispatch;
mod types {
    pub type AccountId = &'static str;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Balance = u128;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Block = crate::support::Block<BlockNumber, Extrinsic>;
    pub type Content = &'static str;
}
pub struct Runtime {
    system: system::SystemModule<Self>,
    balances: balances::BalancesModule<Self>,
    poe: proof_of_existence::POEModule<Self>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Runtime",
            "system",
            &self.system,
            "balances",
            &self.balances,
            "poe",
            &&self.poe,
        )
    }
}
impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}
impl balances::Config for Runtime {
    type Balance = types::Balance;
}
impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}
impl Runtime {
    fn new() -> Self {
        Self {
            system: system::SystemModule::new(),
            balances: balances::BalancesModule::new(),
            poe: proof_of_existence::POEModule::new(),
        }
    }
    fn execute_block(&mut self, block: types::Block) -> Result<(), &'static str> {
        self.system.inc_block_number();
        if block.header.block_number != self.system.block_number() {
            return Err(&"block number does not match what is expected");
        }
        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            let _res = self.dispatch(caller, call).map_err(|e| {
                ::std::io::_eprint(format_args!(
                    "Extrinsic Error\n\tBlock Number: {0}\n\tExtrinsic Number: {1}\n\tError: {2}\n",
                    block.header.block_number, i, e
                ));
            });
        }
        Ok(())
    }
}
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    POE(proof_of_existence::POECall<Runtime>),
}
impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> Result<(), &'static str> {
        self.system.inc_nonce(&caller);
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
            RuntimeCall::POE(call) => {
                self.poe.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}
fn main() {
    let mut runtime = Runtime::new();
    runtime.balances.set_balance(&"alice", 100);
    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                support::Extrinsic {
                    caller: &"alice",
                    call: RuntimeCall::Balances(balances::Call::transfer {
                        to: &"bob",
                        amount: 20,
                    }),
                },
                support::Extrinsic {
                    caller: &"alice",
                    call: RuntimeCall::Balances(balances::Call::transfer {
                        to: &"charlie",
                        amount: 20,
                    }),
                },
            ]),
        ),
    };
    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                support::Extrinsic {
                    caller: &"alice",
                    call: RuntimeCall::POE(proof_of_existence::POECall::CreateClaim {
                        claim: &"Hello, world!",
                    }),
                },
                support::Extrinsic {
                    caller: &"bob",
                    call: RuntimeCall::POE(proof_of_existence::POECall::CreateClaim {
                        claim: &"Hello, world!",
                    }),
                },
            ]),
        ),
    };
    let block_3 = types::Block {
        header: support::Header { block_number: 3 },
        extrinsics: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                support::Extrinsic {
                    caller: &"alice",
                    call: RuntimeCall::POE(proof_of_existence::POECall::RevokeClaim {
                        claim: &"Hello, world!",
                    }),
                },
                support::Extrinsic {
                    caller: &"bob",
                    call: RuntimeCall::POE(proof_of_existence::POECall::CreateClaim {
                        claim: &"Hello, world!",
                    }),
                },
            ]),
        ),
    };
    runtime.execute_block(block_1).expect("invalid block");
    runtime.execute_block(block_2).expect("invalid block");
    runtime.execute_block(block_3).expect("invalid block");
    {
        ::std::io::_print(format_args!("{0:#?}\n", runtime));
    };
}

# Configuring Your Pallet

To build our pallet, we need to include some custom configurations which will allow our pallet to gain access to outside interfaces like:

* Manipulating user balances.
* Generating on-chain randomness.
* Setting limits for how many kitties an single user can own.

We will introduce these to the `trait Config` for our Pallet.

To do this, this we use a few different tools:

* `Currency`: A trait that describes an interface to access and manipulate user balances. Also gives you access to the `Balance` type.
* `Get<u32>`: A trait which simply fetches a `u32` value, allowing the user to configure the `MaxKittiesOwned`.
* `Randomness`: A trait which describes an interface to access an on-chain random value.

We will use these interfaces in the future, but a sneak peak to how you might actually see these used in the code:

```rust
// Make a balance transfer.
T::Currency::transfer(from, to, amount, ExistenceRequirement::KeepAlive)?;

// Get the `MaxKittiesOwned` limit.
let max_kitties: u32 = T::MaxKittiesOwned::get();

// Get a random value.
let random_value = T::KittyRandomness::random(&[]).0;
```

<!-- slide:break -->

<!-- tabs:start -->

#### ** ACTION ITEMS **

Import the `Currency` and `Randomness` traits to your project:

```rust
use frame_support::traits::{Currency, Randomness};
```

Then, update your `trait Config` to have the following:

```rust
// Your Pallet's configuration trait, representing custom external types and interfaces.
#[pallet::config]
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

	/// The Currency handler for the kitties pallet.
	type Currency: Currency<Self::AccountId>;

	/// The maximum amount of kitties a single account can own.
	#[pallet::constant]
	type MaxKittiesOwned: Get<u32>;

	/// The type of Randomness we want to specify for this pallet.
	type KittyRandomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
}
```

#### ** SOLUTION 1 **

This should compile successfully by running:

```bash
cargo build -p pallet-template
```

Don't worry about warnings.

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use frame_support::traits::{Currency, Randomness};

	// The struct on which we build all of our Pallet logic.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/* Placeholder for defining custom types. */

	/* Placeholder for defining custom storage items. */

	// Your Pallet's configuration trait, representing custom external types and interfaces.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The Currency handler for the kitties pallet.
		type Currency: Currency<Self::AccountId>;

		/// The maximum amount of kitties a single account can own.
		#[pallet::constant]
		type MaxKittiesOwned: Get<u32>;

		/// The type of Randomness we want to specify for this pallet.
		type KittyRandomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
	}

	// Your Pallet's events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	// Your Pallet's error messages.
	#[pallet::error]
	pub enum Error<T> {}

	// Your Pallet's callable functions.
	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	// Your Pallet's internal functions.
	impl<T: Config> Pallet<T> {}
}
```

<!-- tabs:end -->

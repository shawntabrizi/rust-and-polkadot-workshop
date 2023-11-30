# Custom Types

We already gave you a sneak peak at defining and using custom types in Substrate.

For our Pallet, we will define an `enum Gender` and a `struct Kitty`.

We will also "import" the `Balance` type into our Pallet, and make it easier to access.

Inside of the `Kitty` struct, we have a unique identifier `dna` which we will use to ensure that each kitty is totally a unique in our blockchain. We will also use this DNA as the seed for generating unique attributes about our kitty!

![Kitty!](./assets/cat-avatar.png)

We also store the `price` of a Kitty with an `Option`. An `Option` can be `Some(value)` or `None`. If the value is `None`, then we will assume the kitty is not for sale.

Finally, note that we take advantage of the `#[derive]` macro to implement all the different traits the Pallet expects from these custom types, just as we explained earlier. If you don't include these, the Rust compiler will start yelling at you as soon as you try to use these custom types.

Check your code against the solution and let's move on to adding storage items for our kitties!

<!-- slide:break-40 -->

<!-- tabs:start -->

#### ** ACTION ITEMS **

Add the following custom types to your Pallet.

```rust
// Allows easy access our Pallet's `Balance` type. Comes from `Currency` interface.
type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// The Gender type used in the `Kitty` struct
#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Gender {
	Male,
	Female,
}

// Struct for holding kitty information
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
#[scale_info(skip_type_params(T))]
pub struct Kitty<T: Config> {
	// Using 16 bytes to represent a kitty DNA
	pub dna: [u8; 16],
	// `None` assumes not for sale
	pub price: Option<BalanceOf<T>>,
	pub gender: Gender,
	pub owner: T::AccountId,
}
```

#### ** SOLUTION 2 **

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

	// The basis which we buil
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Allows easy access our Pallet's `Balance` type. Comes from `Currency` interface.
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// The Gender type used in the `Kitty` struct
	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum Gender {
		Male,
		Female,
	}

	// Struct for holding kitty information
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		// Using 16 bytes to represent a kitty DNA
		pub dna: [u8; 16],
		// `None` assumes not for sale
		pub price: Option<BalanceOf<T>>,
		pub gender: Gender,
		pub owner: T::AccountId,
	}

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

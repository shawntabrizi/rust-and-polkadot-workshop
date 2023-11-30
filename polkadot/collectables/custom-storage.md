# Custom Storage

Our Pallet will use 3 storage items to track all of the state.

1. A `StorageValue` named `CountForKitties` which will keep track of the total number of kitties in the Pallet.
	* This `StorageValue` simply keeps track of a `u64` value that we increment when we generate a new kitty. This means we could have up to `18_446_744_073_709_551_615` kitties.... probably more than we will ever need to worry about.
	* It is worth noting the `ValueQuery` configuration in the `StorageValue`. This basically assumes if there is no value in storage, for example at the start of the network, that we should return the value `0` rather than an option `None`.
2. A `StorageMap` named `Kitties` which will map each kitty to it's unique information.
	* To keep kitties completely unique and easy to look up, the key of our map is the `dna` of the kitty. As such, we cannot have two kitties with the same `dna` since the map will have already been populated by one of them.
	*
3. A `StorageMap` named `KittiesOwned` which will map each user to the list of kitties they own.
	* The key for this storage map will be a user account: `T::AccountID`.
	* The value for this storage map will be a `BoundedVec` with the `dna` of the kitties they own. This will make it easy to then look up each individual kitty for its information since the `dna` is used as the key for the `Kitties` map.
	* By using a `BoundedVec`, we ensure that each storage item has a maximum length, which is important for managing limits within the runtime.

<!-- slide:break-40 -->

<!-- tabs:start -->

#### ** ACTION ITEMS **

Add the following custom storage items to your Pallet.

```rust
/// Keeps track of the number of kitties in existence.
#[pallet::storage]
pub(super) type CountForKitties<T: Config> = StorageValue<_, u64, ValueQuery>;

/// Maps the kitty struct to the kitty DNA.
#[pallet::storage]
pub(super) type Kitties<T: Config> = StorageMap<_, Twox64Concat, [u8; 16], Kitty<T>>;

/// Track the kitties owned by each account.
#[pallet::storage]
pub(super) type KittiesOwned<T: Config> = StorageMap<
	_,
	Twox64Concat,
	T::AccountId,
	BoundedVec<[u8; 16], T::MaxKittiesOwned>,
	ValueQuery,
>;
```

#### ** SOLUTION 3 **

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

	/// Keeps track of the number of kitties in existence.
	#[pallet::storage]
	pub(super) type CountForKitties<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Maps the kitty struct to the kitty DNA.
	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<_, Twox64Concat, [u8; 16], Kitty<T>>;

	/// Track the kitties owned by each account.
	#[pallet::storage]
	pub(super) type KittiesOwned<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		BoundedVec<[u8; 16], T::MaxKittiesOwned>,
		ValueQuery,
	>;

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

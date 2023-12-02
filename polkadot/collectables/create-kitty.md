# Create Kitty

Now that we have set up our custom types and storage items, we can start to write functions that the user can interact with.

We will start with writing a `create_kitty` function so we can mint new kitties into existence!

We have multiple actions to complete to setup our first callable function...

To provide a better user experience for users calling our functions, we can both create custom error messages in case of a failure, and custom events in case of a success.

Our Pallet has some limitations when it comes to creating a new kitty:

1. The user who owns this kitty cannot own too many kitties, ensuring our storage for any individual is bounded.
2. The kitty we create must be unique, and thus cannot have a duplicate `dna` as another kitty.
3. The number of kitties in the whole system cannot overflow a `u64`, else we can't keep track of the number of kitties. However, we already saw that reaching `u64::MAX` is nearly impossible for any blockchain, so not really a big worry.

If our `create_kitty` call is successful, we can emit an event with information about the kitty that was created, and who the owner is. These events can be used by front-end applications to trigger updates to the UX or notify users that things went successfully. Also, block explorers normally index all of the events by default for Substrate chains, so it will be easy to look up when these events happen.

You will also notice that we separate out most of our logic into an **internal** function. This allows us to expose apis in the future, which gives other parts of our runtime low level access to do things like `mint` new kitties. You can see that this internal function does not do any kind of authorization or authentication, but that is fine, because it is an internal function, and only accessible by runtime developers.

<!-- slide:break-40 -->

<!-- tabs:start -->

#### ** 1: ADD ERRORS **

Add the `TooManyOwned`, `DuplicateKitty`, and `Overflow` errors.

These can come up when trying to create a new kitty.

```rust
// Your Pallet's error messages.
#[pallet::error]
pub enum Error<T> {
	/// An account may only own `MaxKittiesOwned` kitties.
	TooManyOwned,
	/// This kitty already exists!
	DuplicateKitty,
	/// An overflow has occurred!
	Overflow,
}
```

#### ** 2: ADD EVENT **

Add the `Created` event to your Pallet.

```rust
// Your Pallet's events.
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
	/// A new kitty was successfully created.
	Created { kitty: [u8; 16], owner: T::AccountId },
}
```

#### ** 3: GEN DNA **

Create an **internal** helper function which generates unique DNA for new kitties.

```rust
// Your Pallet's internal functions.
impl<T: Config> Pallet<T> {
	// Generates and returns DNA and Gender
	fn gen_dna() -> ([u8; 16], Gender) {
		// Create randomness
		let random = T::KittyRandomness::random(&b"dna"[..]).0;

		// Create randomness payload. Multiple kitties can be generated in the same block,
		// retaining uniqueness.
		let unique_payload = (
			random,
			frame_system::Pallet::<T>::extrinsic_index().unwrap_or_default(),
			frame_system::Pallet::<T>::block_number(),
		);

		// Turns into a byte array
		let encoded_payload = unique_payload.encode();
		let hash = frame_support::Hashable::blake2_128(&encoded_payload);

		// Generate Gender
		if hash[0] % 2 == 0 {
			(hash, Gender::Male)
		} else {
			(hash, Gender::Female)
		}
	}
}
```

#### ** 4: MINT KITTY **

Create an **internal** helper function which enables minting new kitties.

```rust
// Helper to mint a kitty
pub fn mint(
	owner: &T::AccountId,
	dna: [u8; 16],
	gender: Gender,
) -> Result<[u8; 16], DispatchError> {
	// Create a new object
	let kitty = Kitty::<T> { dna, price: None, gender, owner: owner.clone() };

	// Check if the kitty does not already exist in our storage map
	ensure!(!Kitties::<T>::contains_key(&kitty.dna), Error::<T>::DuplicateKitty);

	// Performs this operation first as it may fail
	let count = CountForKitties::<T>::get();
	let new_count = count.checked_add(1).ok_or(Error::<T>::Overflow)?;

	// Append kitty to KittiesOwned
	KittiesOwned::<T>::try_append(&owner, kitty.dna)
		.map_err(|_| Error::<T>::TooManyOwned)?;

	// Write new kitty to storage
	Kitties::<T>::insert(kitty.dna, kitty);
	CountForKitties::<T>::put(new_count);

	// Deposit our "Created" event.
	Self::deposit_event(Event::Created { kitty: dna, owner: owner.clone() });

	// Returns the DNA of the new kitty if this succeeds
	Ok(dna)
}
```

#### ** 5: CREATE KITTY **

Finally, create the actual **callable** function.

At this point, really minimal logic since we just call into our helper.

```rust
// Your Pallet's callable functions.
#[pallet::call]
impl<T: Config> Pallet<T> {
	/// Create a new unique kitty.
	///
	/// The actual kitty creation is done in the `mint()` function.
	#[pallet::call_index(0)]
	#[pallet::weight(Weight::default())]
	pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
		// Make sure the caller is from a signed origin
		let sender = ensure_signed(origin)?;

		// Generate unique DNA and Gender using a helper function
		let (kitty_gen_dna, gender) = Self::gen_dna();

		// Write new kitty to storage by calling helper function
		Self::mint(&sender, kitty_gen_dna, gender)?;

		Ok(())
	}
}
```

#### ** SOLUTION 4 **

This should compile successfully by running:

```bash
cargo build -p pallet-template
```

This should compile without warnings.

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use frame_support::traits::{
		fungible::{Inspect, Mutate},
		tokens::Preservation,
		Randomness,
	};

	// The basis which we buil
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Allows easy access our Pallet's `Balance` type. Comes from `Fungible` interface.
	type BalanceOf<T> =
		<<T as Config>::Fungible as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

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

		/// The Fungible handler for the kitties pallet.
		type Fungible: Inspect<Self::AccountId> + Mutate<Self::AccountId>;

		/// The maximum amount of kitties a single account can own.
		#[pallet::constant]
		type MaxKittiesOwned: Get<u32>;

		/// The type of Randomness we want to specify for this pallet.
		type KittyRandomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
	}

	// Your Pallet's events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new kitty was successfully created.
		Created { kitty: [u8; 16], owner: T::AccountId },
	}

	// Your Pallet's error messages.
	#[pallet::error]
	pub enum Error<T> {
		/// An account may only own `MaxKittiesOwned` kitties.
		TooManyOwned,
		/// This kitty already exists!
		DuplicateKitty,
		/// An overflow has occured!
		Overflow,
	}

	// Your Pallet's callable functions.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new unique kitty.
		///
		/// The actual kitty creation is done in the `mint()` function.
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::default())]
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let sender = ensure_signed(origin)?;

			// Generate unique DNA and Gender using a helper function
			let (kitty_gen_dna, gender) = Self::gen_dna();

			// Write new kitty to storage by calling helper function
			Self::mint(&sender, kitty_gen_dna, gender)?;

			Ok(())
		}
	}

	// Your Pallet's internal functions.
	impl<T: Config> Pallet<T> {
		// Generates and returns DNA and Gender
		fn gen_dna() -> ([u8; 16], Gender) {
			// Create randomness
			let random = T::KittyRandomness::random(&b"dna"[..]).0;

			// Create randomness payload. Multiple kitties can be generated in the same block,
			// retaining uniqueness.
			let unique_payload = (
				random,
				frame_system::Pallet::<T>::extrinsic_index().unwrap_or_default(),
				frame_system::Pallet::<T>::block_number(),
			);

			// Turns into a byte array
			let encoded_payload = unique_payload.encode();
			let hash = frame_support::Hashable::blake2_128(&encoded_payload);

			// Generate Gender
			if hash[0] % 2 == 0 {
				(hash, Gender::Male)
			} else {
				(hash, Gender::Female)
			}
		}

		// Helper to mint a kitty
		pub fn mint(
			owner: &T::AccountId,
			dna: [u8; 16],
			gender: Gender,
		) -> Result<[u8; 16], DispatchError> {
			// Create a new object
			let kitty = Kitty::<T> { dna, price: None, gender, owner: owner.clone() };

			// Check if the kitty does not already exist in our storage map
			ensure!(!Kitties::<T>::contains_key(&kitty.dna), Error::<T>::DuplicateKitty);

			// Performs this operation first as it may fail
			let count = CountForKitties::<T>::get();
			let new_count = count.checked_add(1).ok_or(Error::<T>::Overflow)?;

			// Append kitty to KittiesOwned
			KittiesOwned::<T>::try_append(&owner, kitty.dna)
				.map_err(|_| Error::<T>::TooManyOwned)?;

			// Write new kitty to storage
			Kitties::<T>::insert(kitty.dna, kitty);
			CountForKitties::<T>::put(new_count);

			// Deposit our "Created" event.
			Self::deposit_event(Event::Created { kitty: dna, owner: owner.clone() });

			// Returns the DNA of the new kitty if this succeeds
			Ok(dna)
		}
	}
}
```

<!-- tabs:end -->

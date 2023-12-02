# Transfer Kitty

Next we'll need a way for users of this pallet to transfer kitties between accounts.
We'll do this by creating a publicly callable function called `transfer`.

This function will rely on an internal function called `do_transfer` to perform checks and write changes to storage.
It will emit events and handle any errors and handle checks to enforce rules around how a kitty can be transferred.
These include:
* A kitty must exist in order to be transferrable
* A kitty cannot be transferred to its owner
* A kitty cannot be transferred to an account that already has the maximum kitties allowed

To handle errors and events we'll need:

1. `NoKitty`, `NotOwner`, and `TransferToSelf` errors to emit relevant errors when checks do not go through.
1. A `Transferred` event, to emit an event when a transfer is successful.

Then, in `do_transfer` we'll:
1. Check if the kitty exists and isn't being sent to its owner.
1. Update our `KittiesOwned` storage item to reflect the new owners and write all changes to storage.

Finally, we can write the publicly callable dispatchable, `create_kitty` which will simply check the origin of the caller and call `do_transfer`.

<!-- slide:break-40 -->

<!-- tabs:start -->

#### ** 1: ADD ERRORS **

Add the `NoKitty`, `NotOwner`, and `TransferToSelf` errors.

These can come up when trying to transfer a kitty.

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
	/// This kitty does not exist!
	NoKitty,
	/// You are not the owner of this kitty.
	NotOwner,
	/// Trying to transfer or buy a kitty from oneself.
	TransferToSelf,
}
```

#### ** 2: ADD EVENT **

Add the `Transferred` event to your Pallet.

```rust
// Your Pallet's events.
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
	/// A new kitty was successfully created.
	Created { kitty: [u8; 16], owner: T::AccountId },
	/// A kitty was successfully transferred.
	Transferred { from: T::AccountId, to: T::AccountId, kitty: [u8; 16] },
}
```

#### ** 3: DO TRANSFER KITTY **

Create an **internal** helper function which enables transferring new kitties.

```rust
// Update storage to transfer kitty
pub fn do_transfer(
	kitty_id: [u8; 16],
	to: T::AccountId,
) -> DispatchResult {
	// Get the kitty
	let mut kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
	let from = kitty.owner;

	ensure!(from != to, Error::<T>::TransferToSelf);
	let mut from_owned = KittiesOwned::<T>::get(&from);

	// Remove kitty from list of owned kitties.
	if let Some(ind) = from_owned.iter().position(|&id| id == kitty_id) {
		from_owned.swap_remove(ind);
	} else {
		return Err(Error::<T>::NoKitty.into())
	}

	// Add kitty to the list of owned kitties.
	let mut to_owned = KittiesOwned::<T>::get(&to);
	to_owned.try_push(kitty_id).map_err(|_| Error::<T>::TooManyOwned)?;

	// Transfer succeeded, update the kitty owner and reset the price to `None`.
	kitty.owner = to.clone();
	kitty.price = None;

	// Write updates to storage
	Kitties::<T>::insert(&kitty_id, kitty);
	KittiesOwned::<T>::insert(&to, to_owned);
	KittiesOwned::<T>::insert(&from, from_owned);

	Self::deposit_event(Event::Transferred { from, to, kitty: kitty_id });

	Ok(())
}
```

#### ** 4: TRANSFER **

Finally, create the actual **callable** function.

At this point, really minimal logic since we just call into our helper.

```rust
/// Directly transfer a kitty to another recipient.
///
/// Any account that holds a kitty can send it to another Account. This will reset the
/// asking price of the kitty, marking it not for sale.
#[pallet::call_index(1)]
#[pallet::weight(Weight::default())]
pub fn transfer(
	origin: OriginFor<T>,
	to: T::AccountId,
	kitty_id: [u8; 16],
) -> DispatchResult {
	// Make sure the caller is from a signed origin
	let from = ensure_signed(origin)?;
	let kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
	ensure!(kitty.owner == from, Error::<T>::NotOwner);
	Self::do_transfer(kitty_id, to)?;
	Ok(())
}
```

#### ** SOLUTION 5 **

This should compile successfully by running:

```bash
cargo build -p pallet-template
```

There should be no warnings.

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
		/// A kitty was successfully transferred.
		Transferred { from: T::AccountId, to: T::AccountId, kitty: [u8; 16] },
	}

	// Your Pallet's error messages.
	#[pallet::error]
	pub enum Error<T> {
		/// An account may only own `MaxKittiesOwned` kitties.
		TooManyOwned,
		/// This kitty already exists!
		DuplicateKitty,
		/// An overflow has occurred!
		Overflow,
		/// This kitty does not exist!
		NoKitty,
		/// You are not the owner of this kitty.
		NotOwner,
		/// Trying to transfer or buy a kitty from oneself.
		TransferToSelf,
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

		/// Directly transfer a kitty to another recipient.
		///
		/// Any account that holds a kitty can send it to another Account. This will reset the
		/// asking price of the kitty, marking it not for sale.
		#[pallet::call_index(1)]
		#[pallet::weight(Weight::default())]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			kitty_id: [u8; 16],
		) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let from = ensure_signed(origin)?;
			let kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
			ensure!(kitty.owner == from, Error::<T>::NotOwner);
			Self::do_transfer(kitty_id, to)?;
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

		// Update storage to transfer kitty
		pub fn do_transfer(
			kitty_id: [u8; 16],
			to: T::AccountId,
		) -> DispatchResult {
			// Get the kitty
			let mut kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
			let from = kitty.owner;

			ensure!(from != to, Error::<T>::TransferToSelf);
			let mut from_owned = KittiesOwned::<T>::get(&from);

			// Remove kitty from list of owned kitties.
			if let Some(ind) = from_owned.iter().position(|&id| id == kitty_id) {
				from_owned.swap_remove(ind);
			} else {
				return Err(Error::<T>::NoKitty.into())
			}

			// Add kitty to the list of owned kitties.
			let mut to_owned = KittiesOwned::<T>::get(&to);
			to_owned.try_push(kitty_id).map_err(|_| Error::<T>::TooManyOwned)?;

			// Transfer succeeded, update the kitty owner and reset the price to `None`.
			kitty.owner = to.clone();
			kitty.price = None;

			// Write updates to storage
			Kitties::<T>::insert(&kitty_id, kitty);
			KittiesOwned::<T>::insert(&to, to_owned);
			KittiesOwned::<T>::insert(&from, from_owned);

			Self::deposit_event(Event::Transferred { from, to, kitty: kitty_id });

			Ok(())
		}
	}
}
```

<!-- tabs:end -->

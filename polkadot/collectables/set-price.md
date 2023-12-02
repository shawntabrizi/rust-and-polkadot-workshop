# Set Price

To give the ability for a user to set the price of a kitty, we'll need to update the `price` field in our kitty struct and emit in event.
We'll use the [`get`](https://docs.substrate.io/rustdocs/latest/frame_support/storage/trait.StorageMap.html#tymethod.get) and [`insert`](https://docs.substrate.io/rustdocs/latest/frame_support/storage/trait.StorageMap.html#tymethod.insert) methods from our `Kitties` storage map to modify and update the Kitty object.

Just like our other dispatchables, we need to perform a few checks before we allow the caller to write a new price to storage:

* The caller must be a signed origin
* The kitty must already exist
* The caller must be the owner of the kitty

Then, we can simply write the new price to storage and emit a `PriceSet` event.

![Kitty](./assets/kitty1.png)


<!-- slide:break-40 -->

<!-- tabs:start -->

#### ** 1: ADD EVENT **

Add the `PriceSet` event to your Pallet.

```rust
// Your Pallet's events.
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
	/// A new kitty was successfully created.
	Created { kitty: [u8; 16], owner: T::AccountId },
	/// A kitty was successfully transferred.
	Transferred { from: T::AccountId, to: T::AccountId, kitty: [u8; 16] },
	/// The price of a kitty was successfully set.
	PriceSet { kitty: [u8; 16], price: Option<BalanceOf<T>> },
}
```

#### ** 2: SET PRICE **

Finally, create the actual **callable** function.

```rust
/// Set the price for a kitty.
///
/// Updates kitty price and updates storage.
#[pallet::call_index(2)]
#[pallet::weight(Weight::default())]
pub fn set_price(
	origin: OriginFor<T>,
	kitty_id: [u8; 16],
	new_price: Option<BalanceOf<T>>,
) -> DispatchResult {
	// Make sure the caller is from a signed origin
	let sender = ensure_signed(origin)?;

	// Ensure the kitty exists and is called by the kitty owner
	let mut kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
	ensure!(kitty.owner == sender, Error::<T>::NotOwner);

	// Set the price in storage
	kitty.price = new_price;
	Kitties::<T>::insert(&kitty_id, kitty);

	// Deposit a "PriceSet" event.
	Self::deposit_event(Event::PriceSet { kitty: kitty_id, price: new_price });

	Ok(())
}
```

#### ** SOLUTION 6 **

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
		/// The price of a kitty was successfully set.
		PriceSet { kitty: [u8; 16], price: Option<BalanceOf<T>> },
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

		/// Set the price for a kitty.
		///
		/// Updates kitty price and updates storage.
		#[pallet::call_index(2)]
		#[pallet::weight(Weight::default())]
		pub fn set_price(
			origin: OriginFor<T>,
			kitty_id: [u8; 16],
			new_price: Option<BalanceOf<T>>,
		) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let sender = ensure_signed(origin)?;

			// Ensure the kitty exists and is called by the kitty owner
			let mut kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
			ensure!(kitty.owner == sender, Error::<T>::NotOwner);

			// Set the price in storage
			kitty.price = new_price;
			Kitties::<T>::insert(&kitty_id, kitty);

			// Deposit a "PriceSet" event.
			Self::deposit_event(Event::PriceSet { kitty: kitty_id, price: new_price });

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

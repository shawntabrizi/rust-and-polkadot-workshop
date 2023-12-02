
## Implement a storage map for stored items

To add a new claim to the blockchain, the proof-of-existence pallet requires a storage mechanism.
To address this requirement, you can create a key-value map, where each claim points to the owner and the block number when the claim was made.
To create this key-value map, you can use the FRAME [`StorageMap`](https://paritytech.github.io/substrate/master/frame_support/pallet_prelude/struct.StorageMap.html).

To implement storage for the proof-of-existence pallet:

1. Open the `pallets/template/src/lib.rs` file in a text editor.

1. Replace the `#[pallet::storage]` line with the following code block:

	```rust
	#[pallet::storage]
	pub(super) type Claims<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, BlockNumberFor<T>)>;
	```

1. Save your changes.

<!-- slide:break -->

# Solution

```rust
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// Pallets use events to inform users when important changes are made.
	// Event documentation should end with an array that provides descriptive names for parameters.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a claim has been created.
		ClaimCreated { who: T::AccountId, claim: T::Hash },
		/// Event emitted when a claim is revoked by the owner.
		ClaimRevoked { who: T::AccountId, claim: T::Hash },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The claim already exists.
		AlreadyClaimed,
		/// The claim does not exist, so it cannot be revoked.
		NoSuchClaim,
		/// The claim is owned by another account, so caller can't revoke it.
		NotClaimOwner,
	}

	#[pallet::storage]
	pub(super) type Claims<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, BlockNumberFor<T>)>;

	#[pallet::call]    // <-- Step 6. code block will replace this.
}
```


## Implement pallet events

Now that you've configured the pallet to emit events, you are ready to define those events.
As described in [Design the application](#design-the-application), the proof-of-existence pallet emits an event under the following conditions:

- When a new claim is added to the blockchain.
- When a claim is revoked.

Each event also displays an `AccountId` to identify who triggered the
event and the proof-of-existence claim (as `Hash`) that is being stored or removed.

To implement the pallet events:

1. Open the `pallets/template/src/lib.rs` file in a text editor.

1. Replace the `#[pallet::event]` line with the following code block:

	```rust
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
	```

1. Save your changes.

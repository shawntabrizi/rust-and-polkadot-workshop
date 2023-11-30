
## Implement callable functions

The proof-of-existence pallet exposes two callable functions to users:

- `create_claim()` allows a user to claim the existence of a file with a hash.

- `revoke_claim()` allows the owner of a claim to revoke the claim.

These functions use the `StorageMap` to implement the following logic:

- If a claim is already in storage, then it already has an owner and cannot be claimed again.
- If a claim does not in storage, then it is available to be claimed and written to storage.

To implement this logic in the proof-of-existence pallet:

1. Open the `pallets/template/src/lib.rs` file in a text editor.

1. Replace the `#[pallet::call]` line with the following code block:

	```rust
	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::default())]
		pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let sender = ensure_signed(origin)?;

			// Verify that the specified claim has not already been stored.
			ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::AlreadyClaimed);

			// Get the block number from the FRAME System pallet.
			let current_block = <frame_system::Pallet<T>>::block_number();

			// Store the claim with the sender and block number.
			Claims::<T>::insert(&claim, (&sender, current_block));

			// Emit an event that the claim was created.
			Self::deposit_event(Event::ClaimCreated { who: sender, claim });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::default())]
		pub fn revoke_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let sender = ensure_signed(origin)?;

			// Get owner of the claim, if none return an error.
			let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

			// Verify that sender of the current call is the claim owner.
			ensure!(sender == owner, Error::<T>::NotClaimOwner);

			// Remove claim from storage.
			Claims::<T>::remove(&claim);

			// Emit an event that the claim was erased.
			Self::deposit_event(Event::ClaimRevoked { who: sender, claim });
			Ok(())
		}
	}
	```

1. Save your changes and close the file.

1. Check that your code compiles by running the following command:

	```bash
	cargo check -p node-template-runtime
	```

You can refer to the node template [solution](https://github.com/substrate-developer-hub/substrate-node-template/blob/tutorials/solutions/proof-of-existence/pallets/template/src/lib.rs) if you get stuck.

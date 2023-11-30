
## Include pallet errors

The events you defined indicate when calls to the pallet have completed successfully.
Errors indicate when a call has failed, and why it has failed.
For this tutorial, you define the following error conditions:

- An attempt to make a claim that has already exists.

- An attempt to revoke a claim that does not exist.

- An attempt to revoke a claim that is owned by another account.

To implement the errors for the proof-of-existence pallet:

1. Open the `pallets/template/src/lib.rs` file in a text editor.

1. Replace the `#[pallet::error]` line with the following code block:

	```rust
	#[pallet::error]
	pub enum Error<T> {
		/// The claim already exists.
		AlreadyClaimed,
		/// The claim does not exist, so it cannot be revoked.
		NoSuchClaim,
		/// The claim is owned by another account, so caller can't revoke it.
		NotClaimOwner,
	}
	```

1. Save your changes.

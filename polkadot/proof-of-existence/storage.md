
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

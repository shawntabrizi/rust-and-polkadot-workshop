# Add Pallet to Runtime

Now that we have completed our pallet development, we need to enable this pallet in our runtime.

We will need to navigate to and modify a different file than the one we have been working with till now:

```
./substrate-node-template/runtime/src/lib.rs
```

The `pallet-template` was already added to this runtime, but we have made some significant updates throughout this workshop, so we need to update the configuration.

Follow the 2 update steps outlined on this page.

If everything was successful, you should be able to successfully compile your blockchain node:

```bash
cargo build --release
```

# 🎉 CONGRATS 🎉

You have successfully made your first custom Substrate blockchain!

Let's try it out next!

<!-- slide:break -->

1. Add a simple randomness generator to your `runtime/src/lib.rs`:

```rust
pub struct BlockHashRandomness;
impl Randomness<Hash, BlockNumber> for BlockHashRandomness {
	fn random(subject: &[u8]) -> (Hash, BlockNumber) {
		let block_number = frame_system::Pallet::<Runtime>::block_number();
		let block_hash = frame_system::Pallet::<Runtime>::block_hash(block_number);
		let mut subject = subject.to_vec();
		subject.extend_from_slice(&block_hash.0.to_vec());
		let random = <BlakeTwo256 as sp_api::HashT>::hash(&subject[..]);

		return (random, block_number)
	}
}
```

2. Update the `pallet_template::Config` to match the following:

```rust
impl pallet_template::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type KittyRandomness = BlockHashRandomness;
	type MaxKittiesOwned = frame_support::pallet_prelude::ConstU32<100>;
}
```

3. Finally, update the naming of the pallet in the `construct_runtime!` macro to `SubstrateKitties`:

```rust
// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system,
		RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip,
		Timestamp: pallet_timestamp,
		Aura: pallet_aura,
		Grandpa: pallet_grandpa,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		Sudo: pallet_sudo,
		SubstrateKitties: pallet_template, // <-- Update name here
	}
);
```

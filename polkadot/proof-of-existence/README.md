# Proof of Existence Pallet

This tutorial illustrates how to create a custom pallet for a Substrate
runtime using **macros** that are part of the [FRAME](/reference/frame-macros/) development environment.

For this tutorial, you'll build a simple **proof-of-existence** application. Proof-of-existence is an approach to validating the authenticity and ownership of a digital object by storing information about the object on the blockchain.
Because the blockchain associates a timestamp and account with the object, the blockchain record can be used to "prove" that a particular object existed at a specific date and time.
It can also verify who the owner of a record was at that date and time.

## Digital objects and hashes

Instead of storing an entire file on the blockchain, it can be much more efficient to simply store a [cryptographic hash](https://en.wikipedia.org/wiki/Cryptographic_hash_function) of that file.
This is also known as a "digital fingerprint".
The hash enables the blockchain to store files of arbitrary size efficiently by using a small and unique hash value.
Because any change to a file would result in a different hash, users can prove the validity of a file by computing the hash and comparing that hash with the hash stored on chain.

![File Hash](./assets/file-hash.png)

## Build a custom pallet

The Substrate node template has a FRAME-based runtime.
As you learned, FRAME is a library of code that allows you to build a Substrate runtime by composing modules called pallets.
You can think of the pallets as individual pieces of logic that define what your blockchain can do.
Substrate provides you with a number of pre-built pallets for use in FRAME-based runtimes.

![Runtime composition](./assets/frame-runtime.png)

This tutorial demonstrates how to create your own FRAME pallet to be included in your custom blockchain.

<!-- slide:break -->

# Before you begin

1. Make sure you already have Rust for Substrate setup on your computer:

	https://docs.substrate.io/install/


2. Clone the [`substrate-node-template`](https://github.com/substrate-developer-hub/substrate-node-template).

	```bash
	git clone https://github.com/substrate-developer-hub/substrate-node-template
	```

3. Replace the contents of `substrate-node-template/pallets/template/src/lib.rs` with:

	Note: This code will not compile until you complete the full tutorial. The goal is to show you the skeleton of a pallet, and fill in all the parts piece by piece.

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

		#[pallet::config]  // <-- Step 2. code block will replace this.
		#[pallet::event]   // <-- Step 3. code block will replace this.
		#[pallet::error]   // <-- Step 4. code block will replace this.
		#[pallet::storage] // <-- Step 5. code block will replace this.
		#[pallet::call]    // <-- Step 6. code block will replace this.
	}
	```

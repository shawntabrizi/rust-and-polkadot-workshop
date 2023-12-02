# Configure the pallet to emit events

Every pallet has a [Rust "trait"](https://doc.rust-lang.org/book/ch10-02-traits.html) called `Config`.
You use this trait to configure the settings that your specific pallet requires.
For this tutorial, the configuration setting enables the pallet to emit events.

To define the `Config` trait for the proof-of-existence pallet:

1. Open the `pallets/template/src/lib.rs` file in a text editor.

1. Replace the `#[pallet::config]` line with the following code block:

	```rust
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}
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

	#[pallet::event]   // <-- Step 3. code block will replace this.
	#[pallet::error]   // <-- Step 4. code block will replace this.
	#[pallet::storage] // <-- Step 5. code block will replace this.
	#[pallet::call]    // <-- Step 6. code block will replace this.
}
```

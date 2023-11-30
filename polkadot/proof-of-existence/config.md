## Configure the pallet to emit events

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

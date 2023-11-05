# Creating a Balances Module

As mentioned earlier, at the heart of a blockchain is a state machine.

We can create a very naive state machine using simple Rust abstractions, and through this help learn about Rust in the context of blockchains.

We want keep our code organized, so we will not really start building in the `main.rs` file, but actually in separate Rust modules. We can think of the `main.rs` file as glue which brings everything together, and we will see that over the course of this workshop.

## Creating Your First Module

Pretty much every blockchain has a module which handles the balances of users on that blockchain.

This module will tell you: how much balance each user has, provide functions which allow users to transfer those balances, and even some low level functions to allow your blockchain system to manipulate those balances if needed. Think for example if you want to mint new tokens which don't already exist.

This is a great starting point, and the very first module we will build.

1. Create a new file in your `src` folder named `balances.rs`

	```
	touch src/balances.rs
	```

2. In this file, create a `struct`, which will act as the state and entry point for this module:

	```rust
	pub struct BalancesModule {	}
	```

3. Now go back to `src/main.rs`, and import this new module, which will include all the logic inside of it:

	```rust
	mod balances;
	```

4. If we run your program now, you will see it still compiles and runs, but might show you some warnings like:

	```
	warning: struct `BalancesModule` is never constructed
	--> src/balances.rs:1:12
	|
	1 | pub struct BalancesModule {    }
	|            ^^^^^^^^^^^^^^
	|
	= note: `#[warn(dead_code)]` on by default

	warning: `pr` (bin "pr") generated 1 warning
	```

	That's fine! We haven't started using our `BalancesModule` yet, but you can see that the Rust compiler is detecting our new code, and bringing that logic into our main program. This is the start of building our first state machine module.

## Adding State to Our Module

So let's add some simple state to our `balances.rs` module.

We can do this by adding fields into our `BalancesModule` struct.

For a balance system, we really only need to keep track of one thing: how much balance each user has in our system.

For this we will use a `BTreeMap`, which we can import from the Rust `std` library.

Maps are simple `key -> value` objects, allowing us to define an arbitrary sized storage where we can map some user identifier (`key`) to their account balance (`value`).

1. Import the `BTreeMap` object.
	```rust
	use std::collections::BTreeMap;
	```

2. Create a `balances` field in `BalancesModule` using the `BTreeMap`.

	For the `key`, we are using a simple static string for now. This way we can access users like `"alice"`, `"bob"`, etc... This will be changed in the future.

	For the `value`, we will use a `u128`, which is the largest natively supported type in Rust. This will allow our users to have very, very large balances if we want.

	In the end, that looks like:

	```rust
	pub struct BalancesModule {
		balances: BTreeMap<&'static str, u128>,
	}
	```

3. Finally, we need a way to initialize this object and its state. For this, we will implement a function on the `BalancesModule` called `fn new()`:

	```rust
	impl BalancesModule {
		pub fn new() -> Self {
			Self {
				balances: BTreeMap::new()
			}
		}
	}
	```

You can confirm at this point that everything should still be compiling, and that you haven't made any small errors.

Next we will actually start to **use** this module.

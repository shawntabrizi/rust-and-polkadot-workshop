# Interacting With Balances

Now that we have the basics of our `BalancesModule` set up, let's actually interact with it.

For that, we will go back to the `main.rs` file, and create our first `#[test]` which will play with the code we have written so far.

1. In your `src/main.rs` file, add a new test named `fn init_balances()`:

	```rust
	#[test]
	fn init_balances() { }
	```

2. To begin our test, we need to initialize a new instance of our `BalancesModule`:

	```rust
	#[test]
	fn init_balances() {
		let mut balances = balances::BalancesModule::new();
	}
	```

	Note that we make this variable `mut` since we plan to mutate our state using our newly created API.

3. Finally, let's check that our read and write APIs are working as expected:

	```rust
	#[test]
	fn init_balances() {
		let mut balances = balances::BalancesModule::new();

		assert_eq!(balances.balance(&"alice"), 0);
		balances.set_balance(&"alice", 100);
		assert_eq!(balances.balance(&"alice"), 100);
		assert_eq!(balances.balance(&"bob"), 0);
	}
	```

4. We can run this specific test using `cargo test init_balances`, where hopefully you should see that it passes.

I hope at this point you can start to see the beginnings of your simple blockchain state machine.

# Learn Rust

In this section, we will cover the basics of Rust for blockchain development, and get you familiar with patterns and behaviors that you will expect to see when working with the Polkadot SDK.

**This is NOT a replacement for a more formal Rust book or education.**

It is recommended, before you even start this workshop, that you have already read the first 11 chapters of the [Rust Book](https://doc.rust-lang.org/book/).

This section will repeat many of the same ideas, but with a focus on preparing you for working with the Polkadot SDK.

## A Simple State Machine

This entire Rust section will be focused around building a simple State Machine entirely with basic Rust.

At the center of every blockchain is a state machine. This state machine is wrapped in many layers:

- P2P Network
- Database Abstractions
- Transaction Pool
- etc...

However, these abstractions are provided to you automatically by the Polkadot SDK.

So when working with the Polkadot SDK, you really only need to focus on building your state transition function.

To make it easier for you to understand how to do this with the Polkadot SDK, and to introduce you to Rust, we will build a very naive version of a state machine entirely from the ground up in Rust.

Through this process, you will learn:

- How to scaffold and work with basic Rust projects
- How to emulate a simple state machine using Rust objects.
- How to work with safe math in Rust.
- How to do basic error handling.
- How to create various modules for you state machine.
- How to write basic unit tests for your state machine.
- How to abstract and configure these modules with Rust Generics.
- How to interact between modules using public traits.
- How to scaffold your project into various crates which continue to work together.

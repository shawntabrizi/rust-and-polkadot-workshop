# Why Rust?

We are about to embark on the beginning of our journey in actually developing blockchains.

Before we do that though, I think it is important that we understand why this workshop uses Rust, and why Rust is the perfect language for blockchain development.

Rust is perhaps the most impactful evolution of system programming languages since C.

And people seem to love using Rust! It has been voted the [most loved language on StackOverflow](https://github.blog/2023-08-30-why-rust-is-the-most-admired-language-among-developers/) 8 years in a row!

## Memory Safety

Rust's changes the way developers traditionally interact with memory when designing a program.

Usually you have to choose between manual memory management, or using a garbage collector.
However, Rust introduces a third approach known as "ownership."

In Rust, each piece of memory has a single owner.
As the owner, you can either share the data or mutate the data, but never both at the same time.
This ensures that memory is always managed correctly without the overhead of a garbage collector.

The concept of ownership, along with borrowing and lifetimes, enables Rust to achieve memory safety without sacrificing performance.

- Rust's ownership system ensures that memory is always properly managed, preventing memory leaks and dangling pointers, which are common security vulnerabilities in other languages.
- Rust's borrow checker also solves many concurrency bugs by enforcing strict rules for borrowing and returning references, preventing data races, which can lead to unpredictable behavior and security breaches.

## Performance

Ownership is just one example of a zero-cost abstraction that rust has integrated to make your programs better without sacrificing any performance at runtime.

- Rust is designed to be a high-performance language, and it can compete with C and C++ in terms of speed and efficiency. This makes it ideal for low level system, and especially for highly redundant and distributed systems like blockchains.
- Rust's zero-cost abstractions allow developers to write safe and efficient code without sacrificing performance.

## Explicit Handling

Rust also emphasizes that you should be able to easily write code without undefined behaviors.

Rust provides types like `Result` and `Option` allowing you to be able to easily handle errors and never deal with `null` values.

In blockchain systems, undefined behavior can lead to bugs which could result in billions of dollars at risk or lost. While at first it may seem cumbersome to explicitly handle all these conditions, many developers say that they become better programmers after working with Rust as it changes the way you think about designing functions and systems.

## Alternative Execution Environments

Rust has high level of support for alternative VMs and instruction sets.

[Wasm](https://webassembly.org/) and [Risc-V](https://riscv.org/) are two examples of exciting technologies that Blockchain can use for well supported and performant environments to execute state transition functions.

Unlike custom environments like the Ethereum Virtual Machine and other blockchain specific VMs, community adoption of Wasm and Risc-V extend far beyond blockchain.
As a result evolution, tooling, and adoption of these environments are moving forward much faster.

Rust is often at the center of languages that these environments support, and thus investment in learning and using the Rust language is easy to understand.

## Additional Advantages of Rust for Blockchain Development

- Rust's deterministic execution model makes it predictable and easy to reason about, which is important for blockchain development where security and reliability are paramount.
- Rust's support for generics and patterns makes it a versatile and expressive language that can be used to solve a wide variety of problems in blockchain development.
- Rust's macro system allows ecosystems to to extend the Rust language with custom domain specific languages and automatic code generation.

## Additional Content

- https://www.parity.io/blog/why-rust
- https://www.rerun.io/blog/why-rust
- https://github.blog/2023-08-30-why-rust-is-the-most-admired-language-among-developers/

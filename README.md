# rust-and-polkadot-workshop

 A workshop to teach new students the basics of Blockchains, Rust, and the Polkadot SDK.

## Structure

This workshop is designed to take place over 2 days.
The content in this workshop should be detailed enough that users could complete it on their own, but of course the experience is enhanced if there is an expert to help guide students and answer questions as they come.

### Day 1

The first day will consist of lectures about cryptography, blockchain, and Rust, and will end with a Rust workshop.

#### Lectures

1. The course starts with a very basic introduction to [cryptography](pre-rust/crypto/). This will cover:
	1. [Hash Functions](pre-rust/crypto/hash.md)
	2. [Hash Based Structures](pre-rust/crypto/hash-structures.md)
	3. [Digital Signatures](pre-rust/crypto/signatures.md)

2. Then we jump to a basic introduction of [blockchains](pre-rust/blockchain/), covering:
	1. [Bitcoin](pre-rust/blockchain/bitcoin.md)
	2. [Ethereum](pre-rust/blockchain/ethereum.md)
	3. [Blockchain Architecture](pre-rust/blockchain/architecture.md)

3. Finally, we will briefly touch on key concepts in [Rust](pre-rust/why-rust.md)

#### Workshop

1. Then users will really get their hands dirty building an entire blockchain-like [state machine in Rust](rust-state-machine/1/README.md).

### Day 2

In the second day, we will move our focus over to the Polkadot ecosystem, and build fully working blockchains using the Polkadot SDK.

#### Lectures

1. The day starts with an [introduction to Polkadot](pre-polkadot/polkadot-basics.md).
2. Then we learn about [Substrate](pre-polkadot/substrate-basics.md), the blockchain development framework provided by the Polkadot SDK.
3. Finally, we will briefly go over [FRAME](pre-polkadot/frame-basics.md), which is a framework for building state machines for Substrate, very similar to what we designed in the workshop on day 1.

#### Workshops

1. We will then use Substrate and FRAME to build our very first [Proof of Existence Blockchain](polkadot/proof-of-existence/).
2. Then we will follow up with building an [NFT application](polkadot/collectables/) allowing you to buy and sell digital kitties.

## Resources

There are many great resources available for learning about Blockchains, Rust, and Polkadot.

Many of those resources was used directly and indirectly in the creation of this site.

Check out these additional resources if you want to extend or reinforce what you have learned here:

- https://polkadot-blockchain-academy.github.io/pba-book/index.html
- https://substrate.io/
- https://andersbrownworth.com/blockchain/
- https://bitcoin.org/bitcoin.pdf
- https://cryptozombies.io/

## Contributing

This repository is open source and open to contributions.

As of writing this, the content in this tutorial is very fresh, and likely can be greatly improved.

Hopefully in the future we can feel confident about the quality and correctness of everything here.

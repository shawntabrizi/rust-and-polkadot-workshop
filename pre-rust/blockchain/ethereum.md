# Ethereum

Now that you have had a philosophical and technical history of Bitcoin, let's explore the next iteration of blockchains that started with Ethereum.

## From Calculator to Computer

While Bitcoin introduced the concept of a decentralized, trustless currency, Ethereum expanded the scope by introducing programmability.

Ethereum used all the same core principles which powered Bitcoin:

- Blockchain Structure
- Peer-to-peer Networking
- Proof of Work Consensus

However, at the heart of Bitcoin, you could only send and receive a digital currency. Ethereum put at the center of its blockchain system a turing complete execution environment, along with a programming language allowing people to submit and execute their own logic on the blockchain.

The execution environment that Ethereum introduced is called the Ethereum Virtual Machine (EVM). Ethereum uses a stack-based architecture, and introduced the concept of "gas" to limit how much computation a single transaction could do. Solidity was the new programming language that was created specifically for building programs for the EVM.

Programs running on the Ethereum inherited the same properties of decentralization and resilience as Ethereum itself, and lowered the barrier to entry for creating, distributing, and executing decentralized software.

## Smart Contracts

The programs running on Ethereum are called Smart Contracts. Smart contracts have many unique properties which allow them to change the paradigm of software development and consumption.

- Transparency

	Smart contracts, like all data on the blockchain is completely public. This transparency allows for anyone to verify that the software is executing fairly and correctly, unlike privately controlled and centrally executed software.

- Determinism

	Public blockchains must be entirely deterministic to reach consensus with other nodes. Smart contracts on Ethereum also have deterministic execution, meaning that given the same inputs, you will always get the same output.

- Self-Executing

	Once a smart contract is deployed to the blockchain, it is entirely autonomous and self executing. Users who want to interact with a smart contract simply submit transactions to the blockchain and can use the software however the developer intended, and without any additional infrastructure. Everyone who runs the decentralized blockchain acts as a server executing those programs.

- Immutability

	Blockchains have an immutable history due to the recursive linking of the blocks. As such, the smart contracts deployed to Ethereum are also immutable, meaning that in order to stop a program from running, you would need to stop the entire Ethereum network.

- Trustless

	Smart contracts enable users to interact with each other without needing to know or trust the other party explicitly. Instead, they use the predetermined and verifiable properties of smart contracts, and the trustless and permissionless nature of Ethereum to ensure that interactions resolve correctly.

## Decentralized Autonomous Organizations

The Ethereum whitepaper emphasized that the platform's architecture, underpinned by smart contracts and a decentralized blockchain, would enable the creation of Decentralized Autonomous Organizations (DAOs).

In contrast to traditional organizations, DAOs operate on transparent, programmable rules encoded in smart contracts, allowing for collective decision-making by the community. Members of DAOs hold governance tokens, affording them voting rights and influence over proposals.

This innovative approach to organizational structures enables greater inclusivity, transparency, and autonomy, as decisions are executed based on predefined rules of a smart contract without the need for centralized control.

## Proof of Stake

Continued research and development on Ethereum also brought about the idea of Proof of Stake.

Proof of stake is an alternative consensus algorithm to Proof of Work. This approach had the goal of replacing the energy-intensive proof of work process with a more energy-efficient and environmentally friendly alternative.

Rather than spending time and money to calculate hashes, users could put their own tokens/money at stake to participate as a block producer. If the network finds that a block producer is acting maliciously, the chain will slash their tokens. This incentive is used to keep anonymous actors behaving well, and ultimately can achieve the same levels of security consensus as proof of work.

## Layer-2 Scaling

Ethereum also introduced a wave of Layer-2 scaling solutions to address the inherent scalability challenges of running a blockchain like Ethereum.

Layer-2 solutions are "off-chain" protocols that operate alongside the main blockchain. Many Layer-2 solutions are blockchains themselves, but execute their transactions on their own network, which is usually much cheaper than executing them on Ethereum itself.

Eventually, the Layer-2 solutions will submit their blockchain data back to Ethereum in a "rollup", which is a small and cheap to execute summary of the events and transactions that happened on the Layer-2 chain. This reduces the congestion and fees on Ethereum.

Two popular types of rollups are: optimistic rollups and zk-rollups. These rollups are beyond the scope of this workshop.

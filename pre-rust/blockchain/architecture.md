# Blockchain Architecture

We have covered the high level philosophy and innovation behind both Bitcoin and Ethereum.

Let's now look at blockchains in a more technical and architectural way to set up for the exercises we will do later.

## Context: Node

A blockchain node is a single instance of the blockchain software. If you run the Bitcoin client on your computer, you will act as one of many nodes on the network. The decentralized and permissionless nature of nodes makes blockchain systems resilient to attacks.

In these next subsections, we will review architectural concepts which exist within the view of your single node.

### Blocks

In general, you can split up a block into two parts:

- Header: Summary of minimal important information about this block.
- Body: A batched list of state transitions.

![Block Structure](./assets/header-body.svg)

The specifics of the header and body can change depending on the blockchain protocol.

The main reason for the separation between these two parts is that it should be possible to use just the headers to verify certain information about the blockchain. A node using just the headers was described originally in the Bitcoin whitepaper, and is known more commonly as a light client.

Here is a comparison of data found in the header for Bitcoin and Ethereum:

<table>
<th>Bitcoin</th>
<th>Ethereum</th>
<tr>

<td>

- Version
- Previous Hash
- Tx Merkle Root
- Time
- N_Bits
- Nonce

</td>
<td>

- Time
- Block Number
- Base Fee
- Difficulty
- Mix Hash
- Parent Hash
- State Root
- Nonce

</td>
</tr>
</table>

### Transactions

Transactions are signed messages from users of the blockchain indicating what functions they want to execute.

The most common transaction is a transfer of tokens from the signing user to some recipient address.

The transaction format can differ greatly depending on the details of the blockchain, but a few fields are specifically important:

- Signature: The entire transaction should be signed to identify the sender, and prove they want to execute some message.
- Nonce: A unique number to identify the order of a users transactions and prevent replay attacks on the same chain.
- Chain ID: Some unique identifier for the chain the transaction is intended for. This prevents replay attacks across different forks.
- Payload: Some data which identifies what the transaction is intending to do.

Later in this course you will see the term "extrinsic". This is a more general term referring to any kind of external data coming into the blockchain. A transaction is one type of extrinsic, but you can imagine them to be messages which are not signed by normal users, or might have elevated privileges to the blockchain system.

### State Machine

The internals of a blockchain is a simple state machine. It defines:

- The set of valid states
- The rules for transitioning between states.

![State Transition](./assets/state-machine-general.svg)

We can turn a blockchain into a state machine by using the payload in a transaction and the state machine transition function.

Given some initial "genesis state", we can apply transactions to transition the state from block to block.

![Blockchain State Transition](./assets/blockchain-with-state-outside.svg)

It is important to note that the "blockchain" does not contain the state of the chain. This is a common misconception.

The blockchain, as you can see from the definition of the blocks and transactions just contains the information for HOW the state should change.

If you want to construct the blockchain's state, you need to start with some genesis state, and then go block by block and apply the transactions to transition the state. In this way, everyone is able to reconstruct the state of the blockchain on their own.

### Storage

The blockchain state is normally placed in a database on the computer running a node.

Within the database, data is organized into a merkle trie. This allows blockchains to both represent their current state with a single root hash, and also allow other people who do not have a copy of the whole database to verify data in the blockchain state.

For example, imagine you want to know how many tokens you have on the blockchain at a certain point in time.
The only thing you have is the state root of the block.
Someone with the entire blockchain state can provide you with a merkle proof of your account balance, and allow you to verify that information on your own, and with minimal bandwidth and computation used.

We will dive deeper into blockchain storage later in this workshop.

## Context: Network

Now let's take a look at the architecture of blockchains from the context of the decentralized network which powers it.

### Peer-to-Peer Network

A blockchain is not run on a single computer, but instead run on many computers all of which are connected together on a peer-to-per network.

![Peer-to-peer Network](./assets/peer-to-peer.svg ':size=500')

Because of this, there is never a single source of truth for a blockchain. When you are interacting with a blockchain, you are connecting to a single node, and that node is propagating transactions and other information you send to it to the rest of the network.

But each node has its own version and view of the world. Based on its location, network topology, latency, and other physical factors, nodes on the network could have very different views of the truth.


### Forks

Blockchain nodes need to keep track of multiple possible versions of the blockchain all at once.

![Blockchain Forks](./assets/forks.svg)

It could be that any one of these forks ends up being the canonical blockchain, and it is also part of your responsibility as a node on the peer-to-peer network to share the information you are collecting about the blocks, forks, and various states of the chain.

Blockchain nodes will always try to locally verify a block before they put it into the consensus process.
If a block is invalid, it will immediately get rejected locally and not shared with other peers on the network.

![Invalid Forks](./assets/forks-some-invalid.svg)

Invalid blocks can be considered a way to grief the network, but it should be much harder to create invalid blocks than it takes to verify a block is invalid.
And in both Proof of Stake and Proof of Work, the creation of invalid blocks results in a net-loss for the block producer.

### Consensus

Eventually, all nodes should come to consensus with one another about a single canonical history.

There are different mechanisms used to come to consensus. We have described Proof of Work and Proof of Stake as two such systems. Even within Proof of Stake, there are many variants and designs which can alter the behavior of a Proof of Stake system.

Finalization is a term given when the network has some to consensus about the history of the blockchain. Those blocks are deemed "finalized", and there are high security guarantees around the validity and order of the transactions in the block, and that the block is included in the canonical history of the chain.

Just because a transaction is submitted, does not mean that it is guaranteed to get finalized. 
If you were to use cryptocurrency to sell your car to a stranger, you would want to wait until the submitted payment transaction is finalized on the network before you sign any papers or hand over the keys. 

Networks like Bitcoin have non-deterministic finalization due to the properties and rules of Proof of Work. However, because the Bitcoin network is so secure, after 5 or 6 blocks, it is nearly impossible to construct a different and longer chain. As such, you might notice that there is a "confirmation period" that takes place when you make bitcoin transfers.

On Proof of Stake blockchains, you can achieve deterministic finalization, which is a slightly stronger guarantee about the canonical version of a blockchain.

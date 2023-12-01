# Bitcoin

Before there was "blockchain", there was Bitcoin.

Bitcoin does not contain any truly novel software, but instead combined a few existing technologies to solve a problem that had no solution yet.

Let's take a look at that problem, and the construction of Bitcoin.

## The Whitepaper

If you have not already read the Bitcoin Whitepaper, you should:

https://bitcoin.org/bitcoin.pdf

Here is the Abstract:

> Abstract. A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution. Digital signatures provide part of the solution, but the main benefits are lost if a trusted third party is still required to prevent double-spending. We propose a solution to the double-spending problem using a peer-to-peer network. The network timestamps transactions by hashing them into an ongoing chain of hash-based proof-of-work, forming a record that cannot be changed without redoing the proof-of-work. The longest chain not only serves as proof of the sequence of events witnessed, but proof that it came from the largest pool of CPU power. As long as a majority of CPU power is controlled by nodes that are not cooperating to attack the network, they'll generate the longest chain and outpace attackers. The network itself requires minimal structure. Messages are broadcast on a best effort basis, and nodes can leave and rejoin the network at will, accepting the longest proof-of-work chain as proof of what happened while they were gone.

## Internet Native Money

The high level goal which resulted in the creation of Bitcoin was to create an internet native currency.

Since the inception of the internet, people have designed many different kinds of software to solve this problem.

Early attempts, such as e-gold in the late 1990s, aimed to create a digital currency backed by actual gold reserves.
However, centralized control and susceptibility to regulatory pressures led to the eventual demise of e-gold.

Later endeavors, like Digicash and Liberty Reserve, sought to provide digital cash alternatives, yet both faced legal challenges and were eventually shut down.

Around the same time, services like PayPal emerged, providing a centralized but more widely supported digital payment platform.
PayPal successfully facilitated online transactions, but remained tethered to traditional banking systems and relied on conventional currencies.

Ultimately, all digital currencies at the time were in control of, or shut down by, central parties and incumbent powers.

## Problems of Centralization

I don't want to spend to much time recounting the endless history illustrating the issues with centralization and control of power. However, a few recent data points can help illustrate the rise of cryptocurrency and blockchain in our society.

- Financial Crisis of 2008:

	The global financial crisis highlighted the risks associated with centralized banking systems.
	Mismanagement and lack of oversight in major financial institutions led to a widespread economic downturn, affecting millions of individuals who had little control over the decisions that contributed to the crisis.

	https://en.wikipedia.org/wiki/2007%E2%80%932008_financial_crisis

- Government Surveillance Revelations (2013):

	The revelations by Edward Snowden in 2013 exposed extensive global surveillance programs conducted by intelligence agencies.
	The centralized control of such surveillance raised concerns about privacy and individual freedoms, emphasizing the potential misuse of power in centralized authority.

	https://en.wikipedia.org/wiki/2010s_global_surveillance_disclosures

- Equifax Data Breach (2017):

	The Equifax data breach exposed the vulnerabilities of centralized data storage.
	A centralized credit reporting agency being compromised resulted in the unauthorized access of sensitive personal information for nearly 147 million people.

	https://en.wikipedia.org/wiki/2017_Equifax_data_breach

So why not just create decentralized systems?

## Double Spend Attack

The double spend attack was one of the main problems that made creating a decentralized digital currency so difficult.

A double spend attack is when an individual tries to spend the same digital currency unit more than once, exploiting the absence of a centralized authority to prevent such occurrences.

For example:

Imagine Alice has only $50. Using an decentralized internet based digital currency, she tries to spend $30 at two different online stores at the same time.
Both stores feel entitled to their $30 payment, but there is no trivial way to resolve which of these payments is valid, and which is invalid without a central party mediating the situation.

In traditional financial systems, central banks or financial institutions validate transactions and ensure the integrity of the currency to prevent double spending.
But that also means they are in complete control of the system, and are able to act on it in ways which may violate the rights of individuals.

Are there ways to solve this problem while keeping the powers out of the hands of finanical institutions?

## Timestamp Server

One way to solve the dispute of a double spend is to prove which of the two transactions occurred first.
For this, a "timestamp server" can be used to order all transactions, and thus resolve which transactions are valid, and which are invalid.

We can even bundle many transactions into a single object called a "block", order the transactions within the block, and then just put a timestamp on the block as a whole.

However, time is still a "relative" concept. I cannot prove the time that you or I received a specific message.

To create a decentralized timestamp server on a peer-to-peer basis, we need to construct a system for independent actors to come to consensus with one another.

## Proof Of Work

You probably have heard of "proof of work" before. It is the thing that many people talk about when they mention how much power it takes to run the Bitcoin network.

However, this power is being used to run the Proof of Work consensus algorithm.

The concept is quite simple, and starts with the following properties of hash functions:

- It takes computational power to generate a hash.
- Hash functions generate seemingly random output with some given input.
- Hash functions are deterministic, so the same input always generates the same output.

Given this information, imagine you are given the task:

- Construct a block of transactions (which you want to find a decentralized timestamp for).
- Input that block, along with a random input of your choice into a hash function.
- Keep changing the random input until the first 10 digits of your hash are all "0".

	```text
	hash(block + random_number) -> 0x000000000011180838c562c82bb35f3223f46061ebde4a955c27b3f489cf1e03
	```

This task has an interesting property that you, as the person doing the task, have to spend a lot of time and energy to find the random input which will result in the hash you are looking for. This problem becomes exponentially harder as you increase the number of zeros you search for.

However, anyone can quickly and cheaply verify that you completed your task when you reveal the inputs to your hash function since they only need to run the hash function one time to be able to verify your result.

This is where the term "proof of work" comes from: literally a process to easily prove that you have done some heavy work.

## Chaining Blocks

With this mechanism in place, you can actually order blocks of transactions by creating a [hash chain](pre-rust/crypto/hash-structures.md).

For example:

| block 10 | block 11 | block 12 |
| -------- | -------- | -------- |
| hash_9 <br /> txs_10 <br /> nonce_10 | hash_10 <br /> txs_11 <br /> nonce_11 | hash_11 <br /> txs_12 <br /> nonce_12 |

This construction has important properties:

- In order to create block `n`, you must have data provided from block `n-1`, proving that `n-1` existed before `n`.
- The creation of this chain cannot be parallelized, because the construction of each block depends on the random hash of the previous block.
- It is exponentially cheaper to verify this chain of blocks is valid compared to constructing this chain of blocks.

With this structure, we have now created a verifiable ordering system for our transactions.

Thus we have created a "blockchain".

## Permissionless and Decentralized

This process is great because anyone is able to participate in the Proof of Work process.

Even your mobile phone can verify hashes and try to calculate new ones.

This structure becomes a decentralized ledger as soon as you publish your version of a chain to a peer-to-peer network. In this case, anyone can take your blockchain, add new blocks to it, and then publish it again to the network to continue the process.

You can uniquely identify each blockchain by looking at the very first block, called the "genesis block". There is no rules around which blockchains are valid, or what value is represented on each blockchain. Those decisions need to be decided by society itself.

## Consensus

The last problem to solve in this decentralized timestamp server is what to do when there is a conflict in the network.

Imagine that I publish a blockchain with 100 blocks. Both Alice and Bob at the same time create a new `block_101` on top of my blockchain and publish it to the peer-to-peer network. Due to the nature of peer-to-peer networks, the internet, and the real world, there can be lots of propagation delay and network fragmentation about the latest state of the blockchain.

We seem to be back to the same problem as before: how do we pick which of these two blockchains is valid?

The answer is that we follow a simple rule: the blockchain with the most blocks is the real one.

The reason we follow this rule is that the blockchain with the most blocks is also the blockchain which has been exposed to the most people and has "done the most work". It is hard and expensive to build the longest chain, so this is a property that is not easily game-able.

So in the case of Alice and Bob, they both created `block_101`, and we cannot make a decision yet on which of these chains is the right one. What we need to do is wait for the rest of the network to find whichever chain they see as the longest, add their blocks to it, and then keep the chain growing.

Some parts of the network will see Alice's block first, other parts of the network will see Bob's. Eventually, one blockchain will start to grow faster due to the random nature of the process, and block producers will start to ignore shorter blockchains which will eventually die out.

The splitting of a blockchain is called a "fork", and this consensus process is how we can ensure that fork issues are resolved.

## Additional Content

https://andersbrownworth.com/blockchain/

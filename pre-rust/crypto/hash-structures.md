# Hashed Based Data Structures

Using the properties of hash functions, we can construct many different and useful data structures for blockchains.

In many ways, hash based data structures are similar to pointer based data structures like a [linked list](https://en.wikipedia.org/wiki/Linked_list) and a [tree](https://en.wikipedia.org/wiki/Tree_(data_structure)).
In this context, both a pointer and a hash uniquely reference some data in the structure.

Let's take a look at two basic hash based data structures used in blockchain.

## Hash Chains

![Hash Chain](./assets/Hash-Chains.png ':size=500')

A hash chain is a linked list using hashes to connect nodes. Each block contains a reference to the previous hash.

This structure is similar to the structure of a linked list, but with a few unique properties.

1. The structure cannot be cyclic.

	Because each node in the hash chain contains the hash of the previous node, and the hash generation is seemingly random, it is not feasible to construct a chain which is cyclic.

1. The structure can only be formed from start to finish.

	You cannot build new nodes on the hash chain anywhere but at the end of the chain. Again, because the nodes are affected by the hash of the previous node, inserting a new node anywhere else would completely change the data and their corresponding hashes in all subsequent nodes.

> Hint: This is basically the structure of the blockchain itself.

## Merkle Tree

A binary merkle tree is a binary tree using hashes to connect nodes.

![Merkle Tree](./assets/merkle-tree.svg ':size=500')

Raw data we want to store is represented with the data icons at the bottom. All the other black nodes are hashes.

To create this structure, you must:

1. collect all the data you want to place in the merkle tree structure
2. find the hash of each piece of data
3. take two hash nodes and hash them into a new hash node
4. repeat this process for the new nodes until there is a single hash
5. the final hash is the merkle root hash

In this structure you can see that each parent node is a hash representing its two children, and thus any hash can uniquely represent all the data below. You can even include those children node hashes in the parent node like pointers, which would also allow you to traverse down the tree, instead of only from the bottom up.

While this example is specific for a binary merkle tree, you can use the same process to scale the structure to any number of children nodes per parent.

> Hint: This is basically the structure of the data we want to store for a blockchain.

Merkle trees have a number of unique properties which are very relevant for blockchains.

### Merkle Root Hash

At the end of the merkle trie process, you end up with a single hash that uniquely represents all data in the merkle tree.

This is called the merkle root.

You can use the merkle root to quickly compare that two individuals have exactly the same data and structure without having to share all the data itself.
Due to the properties of hashes, if you and I have a merkle tree with the same merkle root, we can be sure that we both have the same children data, which uniquely generates that root hash.
This scales to any amount of data since the final merkle root hash is always the same fixed size.

### Merkle Proofs

Using the merkle tree structure, we can prove the existence of a specific piece of data in the tree without needing all of the data.

Let's say for example I have a merkle tree which is identifiable by its unique merkle root hash.
In that merkle tree, I want to prove to you there is node with the message "Hello, World!".
The naive approach would be to give you all data which is stored in the tree, where you can plainly see "Hello, World!", and let you calculate the whole structure yourself.
However, since the structure uses recursive hashing, whose output cannot be easily controlled, I can provide the same assurance with just a subset of data.

![Merkle Proof](./assets/merkle-proof.svg ':size=500')

This is an image representing the process of a merkle proof.

The only data you need for the merkle proof are represented by red nodes. Grey nodes are data which exists in the full trie, but is not needed for the proof. Black nodes are hashes we can calculate using the red nodes, so also not needed in the proof. Notice how much data is NOT needed!

The green node at the top is the merkle root hash that we just learned about.

Most of the red nodes are just hashes. You can see that those red hashes can uniquely represent a lot of underlying data. Imagine hundreds of terabytes of data, only represented by a few hashes in this tree structure.

You will notice at the bottom right a red data node. For a merkle proof, you will also need to provide the data you want to prove exists in the tree. Starting with that red data node, you can calculate the hash of that data, and generate the black hash node above it.

Then, using the red nodes, you are able to calculate the rest of the black hashes in the tree.

The final piece of the puzzle is the merkle root. Following this process to completion, you will recalculate the merkle root. You can then compare the calculated merkle root to a root provided by a decentralized source, for example a blockchain.

Note that the merkle root is NOT provided in the merkle proof data. You cannot trust the proof to provide a "real merkle root". They could just provide some random hash which works, but is not meaningful. A merkle proof is meaningful when it matches some verified and expected data source.

To re-emphasize, this whole process and proof is possible because of the properties of hashes. It is nearly impossible for someone to construct fake proof data that would match an expected merkle root. Hence, if someone provides a valid merkle proof, it really is a proof that the tree has some data in it.

Merkle proofs reduce the amount of data you need to prove the existence of data from `O(N)` to `O(log N)`. For example, if full tree had 1 billion items in it, you would only need to provide `log2(1_000_000_000) ≈ 30` hashes.

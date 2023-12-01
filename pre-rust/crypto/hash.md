# Hash Functions

We often want a succinct representation of some data with the expectation that we are referring to the same data.

A "fingerprint".

Hash functions can play this role by converting input data of any size into a fixed-size string of characters.

```text
blake2_256("Hello, World!") = 0x511bc81dde11180838c562c82bb35f3223f46061ebde4a955c27b3f489cf1e03
```

The input to a hash function is sometimes referred to as a "pre-image".

This output is commonly referred to as the "hash", "digest", "checksum", or "fingerprint".

These functions are widely used for various purposes, including:

- data integrity verification
- password storage
- digital signatures
- and more!

## Key Properties of Hash Functions

When designing and choosing a hash function, we look for the following properties:

- Can accept unbounded size input.
- Always maps to a bounded, fixed-size output.
- Is fast to compute.

### Cryptographic Hash Functions

There is a subset of hash functions which are called "cryptographic hash functions", which have emphasis on these additional properties:

- Is irreversible, i.e. computable only in one direction
- Pre-image resistance
	- Given a hash value `h`, it should be difficult to find any message `m` such that `h = hash(m)`.
	- This concept is related to that of a one-way function. Functions that lack this property are vulnerable to pre-image attacks.
- Second pre-image resistance (attacker controls one input)
	- Given an input `m1`, it should be difficult to find a different input `m2` such that `hash(m1) = hash(m2)`.
	- This property is sometimes referred to as weak collision resistance. Functions that lack this property are vulnerable to second-preimage attacks.
- Collision resistance (attacker controls both inputs)
	- It should be difficult to find two different messages `m1` and `m2` such that `hash(m1) = hash(m2)`.
		- Such a pair is called a cryptographic hash collision.
	- This property is sometimes referred to as strong collision resistance. It requires a hash value at least twice as long as that required for pre-image resistance; otherwise collisions may be found by a birthday attack.

An example of a cryptographic hash function is Blake2_256.

Learn more at: https://en.wikipedia.org/wiki/Cryptographic_hash_function

### Non-Cryptographic Hash Functions

Non-cryptographic hash functions provide weaker guarantees in exchange for performance. They are OK to use in specific scenarios and when you know that the input is not malicious.

An example of a cryptographic hash function is XXHash. Note the performance increase for XXHash over Blake2:

| Hash Name | Width | Bandwidth (GB/s) | Small Data Velocity | Quality |    Comment    |
|:---------:|:-----:|:----------------:|:-------------------:|:-------:|:-------------:|
| XXH64     | 64    | 19.4 GB/s        | 71.0                | 10      |               |
| Blake2    | 256   | 1.1 GB/s         | 5.1                 | 10      | Cryptographic |

From: https://github.com/Cyan4973/xxHash#benchmarks

## Scale of Hashes

Hash functions can vary in output size, but for the purposes of this illustration let's assume we are working with a cryptographic 256-bit hash like Blake2_256.

There are 2^256 possible unique hashes, which is around 10^77.

The number of atoms in the universe is estimated to be around 10^80 to 10^85.

As you can see, this immense scale allows hash functions to have the key properties listed above which make them secure to use in modern cryptography.

## Hash Functions on Blockchains

Hash functions, like all software, is a constantly evolving space. Different blockchains use different hash functions based on the time they were developed and the properties they were after.

- Bitcoin: SHA2-256 & RIPMD-160
- Ethereum: Keccak-256 (though others supported via EVM)
- Polkadot: Blake2 & xxHash (though others supported via host functions)

## Practical Uses

Hash functions are being used all around you and in nearly all parts of the internet. Here are just a few examples of how they are being used.

1. Data Integrity

	Hash functions are employed to verify the integrity of data. By comparing the hash value of the original data with the hash value calculated after data transfer, users can detect any changes or corruption.

	This property is commonly used with Torrent software.

2. Password Storage

	Hash functions secure passwords by storing only their hash values. During authentication, the system compares the stored hash with the hash of the entered password, eliminating the need to store actual passwords.

3. Digital Signatures

	In digital signatures, hash functions play a role in creating a unique identifier for a piece of data. The hash value is then encrypted with a private key to generate the digital signature.

## Examples

You can find examples to play with hash functions here:

https://www.shawntabrizi.com/substrate-js-utilities/

- Blake2 a String
- XXHash a String

Another great resource is:

https://andersbrownworth.com/blockchain/

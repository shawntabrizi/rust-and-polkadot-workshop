# Digital Signatures

Digital signatures play a vital role in modern cryptography, providing a secure and efficient way to verify the authenticity, integrity, and origin of digital messages and documents.

## Components of a Digital Signature

Digital signatures consist of two key components

1. Private Key

	The private key is known only to the sender and is used for generating the digital signature. It must be kept confidential to maintain the security of the signature.

2. Public Key

	The public key is distributed freely and used by recipients to verify the digital signature. It is crucial for authentication and ensuring the integrity of the message.

## Cryptographic Guarantees

Digital signatures offer several essential properties:

1. Authentication

	Digital signatures verify the identity of the sender, ensuring that the message is genuinely from the claimed source.

2. Integrity

	The digital signature guarantees that the message has not been altered or tampered with during transmission.

3. Non-repudiation

	Non-repudiation prevents the sender from denying their involvement in creating and sending the message.

## Basic Process of Signing Messages

1. Key Generation:

	The sender generates a pair of cryptographic keys – a private key for signing and a public key for verification.

2. Signing:

	The sender uses their private key to apply a mathematical algorithm to the message, creating the digital signature.

3. Verification:

	The recipient uses the sender's public key to verify the signature and confirm the authenticity of the message.

![Signing Process](./assets/sig-verify-flow.svg)

## Replay Attacks

Replay attacks occur when someone intercepts and resends a valid message.
The receiver will carry out the instructions since the message contains a valid signature.

We assume that all channels are insecure, and that all messages should be considered intercepted.

To prevent replay attacks, signing payloads should be designed so that they can only be used one time and in one context.

For example using:

- Using a nonce (a monotonically increasing number)
- Timestamps (or previous blocks)
- Context identifiers like genesis hash and spec versions

## Example

Imagine a software developer wants to releases a new version of their application.

Before distributing the software, the developer digitally signs the installer with their private key.
Users downloading the software can use the developer's public key to verify that the installer is authentic and has not been modified by malicious actors.

Everytime you install a program on your computer, this is happening.

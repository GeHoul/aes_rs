## Coursework information

This is a course assignment implementation of AES-256 in Rust for the purpose of understanding the algorithm's internals and gaining experience at writing Rust code. The implementation uses subfield arithmetic to avoid data-dependent branches and memory accesses.

## working_main (current main is ctr_encrypt_test)

Creates a binary which when run, it promts you to choose between the actions of encryption/decryption. The mode used is simple CTR. The init counter block is stored at the begining of the file during encryption and it's extracted during decryption. This means that decrypting a file with the wrong password destroys the file.

## ctr_encrypt_test

Prints the encrypted output of ctr_encrypt using the [NIST test vectors](https://nvlpubs.nist.gov/nistpubs/legacy/sp/nistspecialpublication800-38a.pdf#page=64) for the CTR mode.

## inv_cipher

Even though CTR mode doesn't use the inverse of the AES-256 cipher function, it was still created and tested.

## sample

Aside from "ctr_encrypt_test", this directory contains "decrypt" and "fake_secret_swap", which were used in the presentation of this coursework to show how the simple CTR mode can be exploited by swapping the encrypted file A with a file B, then decrypt B to gain C and then yield the plain text by xoring A + B + C.

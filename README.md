# c-rgb

This is a very basic library that allows C code to call [`rgb`](https://github.com/rgb-org/rgb)'s
functions to parse and validate RGB transactions (or *proofs* as we generally call them).

The *trickiest* part of this library is memory management: should the C "side" pre-allocate
buffers and then simply ask the Rust "side" to fill them or the other way around?

The current version is implemented keeping in mind these rules:

* All the data structures that might need to be stored or somehow changed in the C side are allocated there.
This includes simple data structures, like `rgb_sha256d`,
`rgb_bitcoin_outpoint`, `rgb_needed_tx`, and some more complex ones like `rgb_contract`, `rgb_proof`.

* Answers to "queries" made from the C side are allocated in Rust: this is because in most cases some
variable-length buffers and arrays are returned (like `Vec<NeededTx> for {contract,proof}.get_needed_txs()`
or `Vec<OP_CODE> for {contract,proof}.get_expected_script()`). In these cases we don't expect the
C side to really do anything with, other than reading, the results (like storing them or sending
them over the network), so we can let Rust allocate and handle them as he prefers. Please remember that
in this case it would still be C's responsibility to manually free the memory allocated by Rust.

**IMPORTANT NOTE:** these rules are the ones that look the most reasonable now. We might change them in the future
if we realize that there are best ways to do it. We don't consider these APIs to be stable at all. Ideally we would
like everything to follow the first rule (allocate the memory in the C side), but this looks kinda complicated in some
cases.

## Build and running the examples
* `./configure`
* `make`

This will build the Rust layer first, and then some binaries in the `examples/build/*` folder that can be run to demonstrate
some features of this library.

These examples are statically-linked so that they can be run without either installing the shared
objects on your system or having to deal with weird `LD_LIBRARY` paths. You might need to install additional
libraries on your system to build the examples, like the static version of `glibc` which might not be available
by default.
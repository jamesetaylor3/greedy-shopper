# greedy-shopper

This package computes which stores to go to and the best way to complete a trip to both minimize travel time and maximize number of items from the user's list. It uses PyO3 to act as a native Python module. It relies on two greedy algorithms, hence the name: greedy-shopper.

### Install nightly rust

PyO3 requires nightly rust; consequently, greedy-shopper requires nightly rust. Assuming you have rustup, run this command to install nightly rust on your machine.

`rustup toolchain install nightly`

To set this project to use this nightly channel of rust, run this command. It won't affect any of your other projects.

`rustup toolchain set nightly`
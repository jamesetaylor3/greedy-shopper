# greedy-shopper

This package computes which stores to go to and the best way to complete a trip to both minimize travel time and maximize number of items from the user's list. It uses PyO3 to act as a native Python module. It relies on two greedy algorithms, hence the name: greedy-shopper.

### Install nightly rust

PyO3 requires nightly rust; consequently, greedy-shopper requires nightly rust. Assuming you have rustup, run this command to install nightly rust on your machine.

`rustup toolchain install nightly`

To set this project to use this nightly channel of rust, run this command in the project directory. It won't affect any of your other projects.

`rustup toolchain set nightly`

### Build

To build the package, run the following command.

`cargo build`

Alternatively, to build an optimized build, run this.

`cargo build --release`

In the target directory, you will find either a debug or a release directory depending on the build you compiled. Inside that directory, find the file libshopper.dylib and rename it to shopper.so. Move the shopper.so file to where you need to use it and you can import the shared library in Python with

`import shopper`

### Todos

- Make get_itenerary_candidates greedy algorithm select all the stores that have the max, not just one at each step
- Optimize all of the cloning
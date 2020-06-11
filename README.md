# greedy-shopper

This package computes which stores to go to and the best way to complete a trip to both minimize travel time and maximize number of items from the user's list. It uses PyO3 to act as a native Python module. It relies on a greedy algorithm, hence the name: greedy-shopper.

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

In the target directory, you will find either a debug or a release directory depending on the build you compiled. Inside that directory, find the file libshopper.dylib and rename it to shopper.so. Move the shopper.so file to where you need to use it.

### Usage

Import into the python program with

```python
import shopper
```

To create a store, use the following as a template. It returs a Store object

```python
store1 = shopper.Store(id='5ec9e03ee9c87eab6a737d7b', inventory=['Bread', 'Toilet Paper'])
```

To create a list of possible iteneraries that each has many items as possible while minimizing number of stores, use this as a template. It returns a list of itenerary objects. Each itenerary object is a collection of the stores to go to.

```python
itenerary_candidates = shopper.get_itenerary_candidates(user_list=['Bread', 'Milk'], stores_py=[store1, store2])
```

To find the fastest itenerary and the fastest path out of all of the stores in the itenerary, use the following as a template. The parameter, matrix, is a dictionary with nested dictionaries that stores the travel durations between any two places in the itenerary candidates. The matrix must be configured such that `matrix[id1][id2] == matrix[id2][id1]` and evaluates to the travel time between the two stores represented by their respected ids, id1 and id2. The function, solve_trip, returns a trip object.

```python
best_trip = shopper.solve_trip(itenerary_candidates=itenerary_candidates, matrix=matrix)
```

The trip object has three get-only (no setting!) attributes worth knowing.

`Trip.path` is an array of Store objects in the best order to go navigate them. It has n elements if the number of stores in the itenerary candidates are all n.

`Trip.stop_times` is an array of floats where the i'th element is the time it will take to get from the store[i - 1] to store[i]. The first element is the time to get from the users current location to the first stop. This array has length of n + 1 because the last element is the time to return home.

`Trip.total_time` is the sum of all elements in `Trip.stop_times`

### License

greedy-shopper is licensed under the [MIT License](https://opensource.org/licenses/MIT). PyO3 is licensed under the [Apache-2.0 license](https://opensource.org/licenses/APACHE-2.0). rust-itertools is licensed under the [MIT License](https://opensource.org/licenses/MIT). Rust is licensed under the [MIT License](https://opensource.org/licenses/MIT). Python is licensed under the [Python License](https://docs.python.org/3.8/license.html).
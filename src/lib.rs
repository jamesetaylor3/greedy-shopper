use std::collections::HashSet;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::hash::{Hash, Hasher};

#[pyclass]
#[text_signature = "(id, inventory, /)"]
#[derive(Clone, Debug)]
struct Store {
	index: usize,
	id: String,
	inventory: HashSet<String>,
}

#[pymethods]
impl Store {
	#[new]
	fn new(index: usize, id: String, inventory: HashSet<String>) -> Self {
		Store { index, id, inventory }
	}
}

impl PartialEq for Store {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index
	}
}

impl Eq for Store {}

impl Hash for Store {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.index.hash(state);
	}
}

#[derive(Debug)]
struct Trip {
	path: Vec<String>,
	stops_distance: Vec<f64>,
	total_distance: f64,
}

impl Trip {
	fn new() -> Self {
		let path = Vec::new();
		let stops_distance = Vec::new();
		let total_distance = 0.0;
		Trip { path, stops_distance, total_distance }
	}

	fn add_stop(&mut self, store_id: String, distance: f64) {
		self.path.push(store_id);
		self.stops_distance.push(distance);
		self.total_distance += distance;
	}

	fn add_home_distance(&mut self, distance: f64) {
		self.stops_distance.push(distance);
		self.total_distance += distance;
	}
}


#[pyfunction]
#[text_signature = "(user_list, stores_py, matrix, /)"]
fn solve(user_list: HashSet<String>, stores_py: Vec<&PyCell<Store>>, matrix: Vec<Vec<f64>>) -> PyResult<()> {
	let stores: HashSet<Store> = stores_py.iter().map(|s| s.extract().unwrap()).collect();

	let mut itenerary_candidates: Vec<HashSet<Store>> = Vec::new();

	/*
		A couple notes about what is going on here.
		Use greedy algorithm to get list of stores that we need
		Maybe use accurate, combinatorical one if the number of stores is small enough
	*/

	// for i in 1..stores.len() {
	// 	let mut early_stop = false;
	// 	if early_stop {
	// 		break;
	// 	}
	// }

	/*
		This uses navigation by means of a greedy algorithm. Not most efficient way.
		Maybe use a different one.
	*/

	itenerary_candidates.push(stores);

	let mut best_trip = Trip::new();
	best_trip.total_distance = 1000.;  // need better way of doing this

	for iten in itenerary_candidates.iter_mut() {
		let mut curr_loc_index = matrix.len() - 1;

		let mut trip = Trip::new();

		loop {
			let mut closest_store = Store::new(0, String::new(), HashSet::new());  // find better way of doing these two lines
			let mut min_dist = 1000.;

			for store in iten.iter() {
				let dist = *matrix.get(curr_loc_index).unwrap().get(store.index).unwrap();
				if dist < min_dist {
					min_dist = dist;
					closest_store = store.clone();  // must be cloned to detach immutability reference from iten
				}
			}

			trip.add_stop(String::from(&closest_store.id), min_dist);

			curr_loc_index = closest_store.index;

			iten.remove(&closest_store);

			if iten.len() == 0 {
				trip.add_home_distance(matrix[curr_loc_index][matrix.len() - 1]);

				if best_trip.total_distance > trip.total_distance {
					best_trip = trip;
				}

				break;
			}
		}
	}

	println!("{:?}", best_trip);


	println!("{:?}", user_list);
	println!("{:?}", stores);
	println!("{:?}", matrix);

	Ok(())
}

#[pymodule]
fn shopper(_py: Python, m: &PyModule,) -> PyResult<()> {
	m.add_class::<Store>()?;
	m.add_wrapped(wrap_pyfunction!(solve)).unwrap();

	Ok(())
}
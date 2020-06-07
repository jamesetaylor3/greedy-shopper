mod store;
mod itenerary;
mod trip;

use std::collections::HashSet;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use store::*;
use itenerary::*;
use trip::*;

/**
	This uses a greedy algorithm to determine which stores in the area to go to.
**/

#[pyfunction]
#[text_signature = "(user_list, stores_py)"]
fn get_itenerary_candidates(user_list: HashSet<String>, stores_py: Vec<&PyCell<Store>>) -> PyResult<Vec<Itenerary>> {
	let stores: HashSet<Store> = stores_py.iter().map(|s| s.extract().unwrap()).collect();
	let mut user_list = user_list;

	let mut itenerary_candidates: Vec<Itenerary> = Vec::new();

	let mut itenerary = Itenerary::new();

	// maybe also use for loop that limits at five stores
	while user_list.len() != 0 {
		// need to really figure out a better way to syntatically put this. could use options.
		let mut best_store: Store = Store::new(0, String::new(), HashSet::new());
		let mut best_match: usize = 0;

		for store in stores.iter() {
			let match_score = store.inventory.intersection(&user_list).cloned().collect::<Vec<String>>().len();

			if match_score == user_list.len() {
				let mut i = itenerary.clone();
				i.add_store(&best_store);
				itenerary_candidates.push(i);

			} else if match_score > best_match {
				best_store = store.clone();
				best_match = match_score;
			}
		}

		itenerary.add_store(&best_store);

		user_list = user_list.difference(&best_store.inventory).cloned().collect();
	}

	Ok(itenerary_candidates)
}


/*
	This uses navigation by means of a greedy algorithm. Not most efficient way.
	Because the number of stores with be relatively small (<5), we can use the
	exact solution to this travelling salesperson problem.
*/

#[pyfunction]
#[text_signature = "(itenerary_candidates, matrix, /)"]
fn solve_trip(itenerary_candidates: Vec<HashSet<Store>>, matrix: Vec<Vec<f64>>) -> PyResult<Trip> {
	let mut itenerary_candidates = itenerary_candidates;

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

	println!("{:?}", matrix);

	Ok(best_trip)
}

#[pymodule]
fn shopper(_py: Python, m: &PyModule,) -> PyResult<()> {
	m.add_class::<Store>()?;
	m.add_wrapped(wrap_pyfunction!(get_itenerary_candidates)).unwrap();
	m.add_wrapped(wrap_pyfunction!(solve_trip)).unwrap();

	Ok(())
}
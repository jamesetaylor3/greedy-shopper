mod store;
mod itenerary;
mod trip;

use pyo3::prelude::*;
use itertools::Itertools;
use std::collections::HashSet;
use pyo3::wrap_pyfunction;
use store::*;
use itenerary::*;
use trip::*;

/**
	This uses a greedy algorithm to determine which stores in the area to go to.
	Need to make it automatically stop if we hit five or so stores.
**/

#[pyfunction]
#[text_signature = "(user_list, stores_py)"]
fn get_itenerary_candidates(user_list: HashSet<String>, stores_py: Vec<&PyCell<Store>>) -> PyResult<Vec<Itenerary>> {
	let stores: Vec<Store> = stores_py.iter().map(|s| s.extract().unwrap()).collect();

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


/**
	Solves traveling salesperson problem with brute force. This is okay, because the maximum numbers of
	stores is five, a highly reasonable amount.
**/

#[pyfunction]
#[text_signature = "(itenerary_candidates, matrix, /)"]
fn solve_trip(itenerary_candidates: Vec<Itenerary>, matrix: Vec<Vec<f64>>) -> PyResult<Trip> {
	let num_stores = itenerary_candidates.get(0).expect("Need at least one itenerary candidate").stores.len();
	let home_index = matrix.len() - 1;

	let mut best_trip = Trip::new();
	best_trip.total_distance = 1000.;  // need better way of doing this

	for iten in itenerary_candidates.iter() {
		for path in iten.stores.iter().combinations(num_stores) {

			let mut curr_loc_index = home_index;
			let mut trip = Trip::new();

			for store in path.iter() {
				let dist = *matrix.get(curr_loc_index).unwrap().get(store.index).unwrap();

				trip.add_stop(String::from(&store.id), dist);

				curr_loc_index = store.index;
			}

			let dist = *matrix.get(curr_loc_index).unwrap().get(home_index).unwrap();

			trip.add_home_distance(dist);

			if trip.total_distance < best_trip.total_distance {
				best_trip = trip;
			}
		}
	}

	Ok(best_trip)
}

#[pymodule]
fn shopper(_py: Python, m: &PyModule,) -> PyResult<()> {
	m.add_class::<Store>()?;
	m.add_wrapped(wrap_pyfunction!(get_itenerary_candidates)).unwrap();
	m.add_wrapped(wrap_pyfunction!(solve_trip)).unwrap();

	Ok(())
}
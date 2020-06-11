mod store;
mod itenerary;
mod trip;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use itertools::Itertools;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;
use store::*;
use itenerary::*;
use trip::*;

/**
	This uses a greedy algorithm to determine which stores in the area to go to.
	Need to make it automatically stop if we hit five or so stores.
**/

#[pyfunction]
#[text_signature = "(user_list, stores_py)"]
fn get_itenerary_candidates(user_list: Vec<String>, stores_py: Vec<&PyCell<Store>>) -> PyResult<Vec<Itenerary>> {
	let user_list: HashSet<String> = HashSet::from_iter(user_list.iter().cloned());
	let stores: HashSet<Store> = stores_py.iter().map(|s| s.extract().unwrap()).collect();

	let mut itenerary_candidates: Vec<Itenerary> = Vec::new();

	itenerary_candidates.push(Itenerary::new());

	let mut keepers: Vec<Itenerary> = Vec::new();

	loop {
		match itenerary_candidates.pop() {
			Some(iten) => {
				if iten.stores.len() == 5 {
					itenerary_candidates.push(iten);
					break;
				}

				let items_left = user_list.difference(&iten.items_covered).cloned().collect::<HashSet<_>>();

				if items_left.len() == 0 {
					itenerary_candidates.push(iten);
					break;
				}

				let mut best_match: usize = 0;

				for store in stores.difference(&iten.stores).collect::<HashSet<_>>().iter() {
					let match_score = store.inventory.intersection(&items_left).cloned().collect::<Vec<_>>().len();

					if match_score > best_match {
						if keepers.len() != 0 {
							keepers.clear();
						}

						best_match = match_score;

						let mut i = iten.clone();
						i.add_store(&store);
						keepers.push(i);

					} else if match_score == best_match && best_match != 0 {
						let mut i = iten.clone();
						i.add_store(&store);
						keepers.push(i);
					}
				}

				if best_match == 0 {
					itenerary_candidates.push(iten);
					break;
				}
			},
			None =>  {
				itenerary_candidates = keepers.clone();
				keepers.clear();
			},
		}
	}

	Ok(itenerary_candidates)

}


/**
	Solves traveling salesperson problem with brute force. This is okay, because the maximum numbers of
	stores is five, a highly reasonable amount.
**/

#[pyfunction]
#[text_signature = "(itenerary_candidates, matrix, /)"]
fn solve_trip(itenerary_candidates: Vec<Itenerary>, matrix: HashMap<String, HashMap<String, f64>>) -> PyResult<Trip> {
	let num_stores = itenerary_candidates.get(0).expect("Need at least one itenerary candidate").stores.len();

	let mut best_trip = Trip::new();
	best_trip.total_time = 1000000.;  // need better way of doing this

	for iten in itenerary_candidates.iter() {
		for path in iten.stores.iter().combinations(num_stores) {

			let mut curr_loc = String::from("HOME");
			let mut trip = Trip::new();

			for store in path.iter() {
				let dist = *matrix.get(&curr_loc).unwrap().get(&store.id).unwrap();

				trip.add_stop(String::from(&store.id), dist);

				curr_loc = store.id.clone();
			}

			let dist = *matrix.get(&curr_loc).unwrap().get("HOME").unwrap();

			trip.add_home_distance(dist);

			if trip.total_time < best_trip.total_time {
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
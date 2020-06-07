use pyo3::prelude::*;
use std::collections::HashSet;
use super::store::*;

#[pyclass]
#[derive(Clone)]
pub struct Itenerary {
	pub stores: Vec<Store>,
	items_covered: HashSet<String>,
}

impl Itenerary {
	pub fn new() -> Self {
		let stores: Vec<Store> = Vec::new();
		let items_covered: HashSet<String> = HashSet::new();

		Itenerary { stores, items_covered }
	}

	#[inline]
	pub fn add_store(&mut self, store: &Store) {
		// this desparately needs to be implemented
		self.stores.push(store.clone());
		self.items_covered = self.items_covered.union(&store.inventory).cloned().collect();
	}
}
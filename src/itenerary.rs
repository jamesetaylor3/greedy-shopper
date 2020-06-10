use pyo3::prelude::*;
use std::collections::HashSet;
use super::store::*;

#[pyclass]
#[derive(Clone)]
pub struct Itenerary {
	#[pyo3(get)]
	pub stores: HashSet<Store>,
	pub items_covered: HashSet<String>,
}

impl Itenerary {
	pub fn new() -> Self {
		let stores: HashSet<Store> = HashSet::new();
		let items_covered: HashSet<String> = HashSet::new();

		Itenerary { stores, items_covered }
	}

	#[inline]
	pub fn add_store(&mut self, store: &Store) {
		self.stores.insert(store.clone());
		self.items_covered = self.items_covered.union(&store.inventory).cloned().collect();
	}
}
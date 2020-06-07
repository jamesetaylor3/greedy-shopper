use pyo3::prelude::*;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[pyclass]
#[text_signature = "(id, inventory, /)"]
#[derive(Clone, Debug)]
pub struct Store {
	pub index: usize,
	pub id: String,
	pub inventory: HashSet<String>,
}

#[pymethods]
impl Store {
	#[new]
	pub fn new(index: usize, id: String, inventory: HashSet<String>) -> Self {
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
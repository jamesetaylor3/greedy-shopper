use pyo3::prelude::*;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[pyclass]
#[text_signature = "(id, inventory, /)"]
#[derive(Clone)]
pub struct Store {
	#[pyo3(get)]
	pub id: String,
	#[pyo3(get)]
	pub inventory: HashSet<String>,
}

#[pymethods]
impl Store {
	#[new]
	pub fn new(id: String, inventory: HashSet<String>) -> Self {
		Store { id, inventory }
	}
}

impl PartialEq for Store {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for Store {}

impl Hash for Store {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}
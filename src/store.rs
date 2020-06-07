use pyo3::prelude::*;
use std::collections::HashSet;

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
use std::collections::HashSet;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass]
#[text_signature = "(id, inventory, /)"]
#[derive(Clone, Debug)]
struct Store {
	id: String,
	inventory: HashSet<String>,
}

#[pymethods]
impl Store {
	#[new]
	fn new(id: String, inventory: HashSet<String>) -> Self {
		Store { id, inventory }
	}
}

struct Itenerary {

}

#[pyfunction]
#[text_signature = "(stores, matrix, /)"]
fn solve(user_list: HashSet<String>, stores_py: Vec<&PyCell<Store>>, matrix: Vec<Vec<f64>>) -> PyResult<()> {
	let stores: Vec<Store> = stores_py.iter().map(|s| s.extract().unwrap()).collect();

	let mut itenerary_candidates: HashSet<Itenerary> = HashSet::new();

	for i in 1..stores.len() {
		
	}

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
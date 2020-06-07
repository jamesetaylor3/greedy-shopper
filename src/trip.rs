use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct Trip {
	pub path: Vec<String>,
	pub stops_distance: Vec<f64>,
	pub total_distance: f64,
}

impl Trip {
	pub fn new() -> Self {
		let path = Vec::new();
		let stops_distance = Vec::new();
		let total_distance = 0.0;
		Trip { path, stops_distance, total_distance }
	}

	#[inline]
	pub fn add_stop(&mut self, store_id: String, distance: f64) {
		self.path.push(store_id);
		self.stops_distance.push(distance);
		self.total_distance += distance;
	}

	#[inline]
	pub fn add_home_distance(&mut self, distance: f64) {
		self.stops_distance.push(distance);
		self.total_distance += distance;
	}
}
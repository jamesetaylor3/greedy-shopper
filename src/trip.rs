use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct Trip {
	#[pyo3(get)]
	pub path: Vec<String>,
	#[pyo3(get)]
	pub stop_times: Vec<f64>,
	#[pyo3(get)]
	pub total_time: f64,
}

impl Trip {
	pub fn new() -> Self {
		let path = Vec::new();
		let stop_times = Vec::new();
		let total_time = 0.0;
		Trip { path, stop_times, total_time }
	}

	#[inline]
	pub fn add_stop(&mut self, store_id: String, distance: f64) {
		self.path.push(store_id);
		self.stop_times.push(distance);
		self.total_time += distance;
	}

	#[inline]
	pub fn add_home_distance(&mut self, distance: f64) {
		self.stop_times.push(distance);
		self.total_time += distance;
	}
}
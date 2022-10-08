use std::{collections::HashMap, fmt::Error};

pub struct Store {
	data: HashMap<String, String>,
}

impl Store {
	pub fn new() -> Self {
		Self {
			data: HashMap::new(),
		}
	}
	pub fn get(&self, key: String) -> String {
		let value = self.data.get(&key).unwrap().to_owned();

		value
	}
	pub fn put(&mut self, key: String, value: String) -> Result<(), Error> {
		self.data.insert(key, value);

		Ok(())
	}
	pub fn delete(&mut self, key: String) -> Option<String> {
		self.data.remove(&key)
	}
}

impl Default for Store {
	fn default() -> Self {
		Self::new()
	}
}

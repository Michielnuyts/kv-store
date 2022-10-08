#![recursion_limit = "512"]
use std::sync::Mutex;

use rocket::{launch, response::status::NotFound, routes, State};
#[macro_use]
extern crate rocket;

mod store;

#[put("/<key>", format = "json", data = "<value>")]
fn key_value_put_handler(
	key: &str,
	value: String,
	shared: &State<SharedData>,
) -> Result<(), NotFound<String>> {
	let shared_data: &SharedData = shared.inner();
	let mut store = shared_data.store.lock().unwrap();

	store.put(key.into(), value)
		.map_err(|err| NotFound(err.to_string()))
}

#[get("/<key>")]
fn key_value_get_handler(
	key: &str,
	shared: &State<SharedData>,
) -> Result<String, NotFound<String>> {
	let shared_data: &SharedData = shared.inner();
	let store = shared_data.store.lock().unwrap();

	Ok(store.get(key.into()))
}

struct SharedData {
	store: Mutex<store::Store>,
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.manage(SharedData {
			store: Mutex::new(store::Store::new()),
		})
		.mount("/v1", routes![key_value_put_handler, key_value_get_handler])
}

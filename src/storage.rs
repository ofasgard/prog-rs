use crate::clock::ProgressClock;
use crate::clock::ProgressClockMutex;
use crate::render::add_clock;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ Document, HtmlInputElement, HtmlAnchorElement, File, Storage };

use std::sync::Arc;
use std::sync::Mutex;
use std::ops::Deref;
use serde_json::json;

pub fn export_clocks(document: &Document, clocks_mx: &Arc<Mutex<Vec<ProgressClockMutex>>>) {
	// Create an invisible anchor element.
	let href = document.create_element("a").unwrap();
	let href : HtmlAnchorElement = href.dyn_into::<HtmlAnchorElement>().map_err(|_| ()).unwrap();
	
	// Create a non-mutex copy of the current clocks.
	let mut clocks : Vec<ProgressClock> = Vec::new();
	for clock_mx in &*clocks_mx.lock().expect("Failed to obtain a lock on state mutex!") {
		let clock_handle = clock_mx.lock().expect("Failed to obtain a lock on a progress clock!");
		let clock : ProgressClock = clock_handle.deref().clone();
		clocks.push(clock);
	}
	
	// Serialize to JSON.
	let serialized_clocks = json!(clocks);
	
	// Build as a data URL and update the anchor.
	let link = format!("data:text/plain;charset=utf-8,{}", serialized_clocks);
	href.set_href(&link);
	href.set_download("progress_clocks.json");
	
	// Simulate a click on the anchor, then remove it.
	let body = document.body().expect("document should have a body");
	body.append_child(&href).unwrap();
	
	href.click();
	href.remove();
}

pub fn import_clocks(document: &Document, clocks_mx: &Arc<Mutex<Vec<ProgressClockMutex>>>) {
	// Retrieve the import dialog (where the opened file lives).
	let import_dialog = document.get_element_by_id("import-input").unwrap();
	let import_dialog : HtmlInputElement = import_dialog.dyn_into::<HtmlInputElement>().map_err(|_| ()).unwrap();

	// Select the first file.
	let files = import_dialog.files().unwrap();
	let file : File = files.item(0).unwrap();
	
	// File access is asynchronous and async closures aren't supported, so we must use `spawn_local()` here.
	let doc = document.clone();
	let cm = Arc::clone(clocks_mx);
	wasm_bindgen_futures::spawn_local(async move {
		// Follow the standard process to convert a JS Promise into a Rust future, and get the result.
		let text_promise = file.text();
		let result = JsFuture::from(text_promise).await.unwrap();
		
		assert!(result.is_instance_of::<JsValue>());
		let text : JsValue = result.dyn_into().unwrap();
		let text_str = text.as_string().unwrap();
		
		// Deserialize the resulting JSON as a vector of `ProgressClock` objects.
		let clocks : Vec<ProgressClock> = serde_json::from_str::<Vec<ProgressClock>>(&text_str).unwrap();
		for clock in clocks {
			// Use the existing function to add them to the app.
			add_clock(Some(clock.clone()), &doc, &cm, clock.is_positive());
		}
	});
}

pub fn save_clocks(storage: &Storage, clocks_mx: &Arc<Mutex<Vec<ProgressClockMutex>>>) {
	// Create a non-mutex copy of the current clocks.
	let mut clocks : Vec<ProgressClock> = Vec::new();
	for clock_mx in &*clocks_mx.lock().expect("Failed to obtain a lock on state mutex!") {
		let clock_handle = clock_mx.lock().expect("Failed to obtain a lock on a progress clock!");
		let clock : ProgressClock = clock_handle.deref().clone();
		clocks.push(clock);
	}
	
	// Serialize the clocks and save to storage.
	let serialized_clocks = json!(clocks).to_string();
	storage.set_item("prog_rs_clocks", &serialized_clocks).expect("Failed to write to local storage!");
}

pub fn load_clocks(document: &Document, storage: &Storage, clocks_mx: &Arc<Mutex<Vec<ProgressClockMutex>>>) {
	let serialized_clocks_maybe = match storage.get_item("prog_rs_clocks") {
		Ok(x) => x,
		Err(_) => return
	};
	
	if serialized_clocks_maybe.is_none() { return; }
	let serialized_clocks = serialized_clocks_maybe.unwrap();
	
	let clocks : Vec<ProgressClock> = serde_json::from_str::<Vec<ProgressClock>>(&serialized_clocks).unwrap();
	for clock in clocks {
		// Use the existing function to add them to the app.
		add_clock(Some(clock.clone()), document, clocks_mx, clock.is_positive());
	}
}

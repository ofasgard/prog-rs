pub mod clock;

use crate::clock::ProgressClock;

use wasm_bindgen::prelude::*;
use web_sys::{ HtmlInputElement, HtmlDivElement, PointerEvent };
use console_error_panic_hook;

use uuid::Uuid;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
	let mut clocks: Vec<ProgressClock> = Vec::new();

	// Get a handle to the document.
	let window = web_sys::window().expect("no window function");
	let document = window.document().expect("should have a document on window");
	
	// Retrieve elements from the page.
	let positive_button = document.get_element_by_id("add-positive").unwrap();
	let positive_button : HtmlInputElement = positive_button.dyn_into::<HtmlInputElement>().map_err(|_| ()).unwrap();
	
	let negative_button = document.get_element_by_id("add-negative").unwrap();
	let negative_button : HtmlInputElement = negative_button.dyn_into::<HtmlInputElement>().map_err(|_| ()).unwrap();
	
	let clock_area = document.get_element_by_id("clock-area").unwrap();
	let clock_area : HtmlDivElement = clock_area.dyn_into::<HtmlDivElement>().map_err(|_| ()).unwrap();
	
	// Set up event handlers for "add clock" buttons.
	let doc = document.clone();
	let ca = clock_area.clone();
	let positive_button_handler = Closure::<dyn FnMut(_)>::new(move |e: PointerEvent| {
		let div = doc.create_element("div").unwrap();
		div.set_id(Uuid::new_v4().to_string().as_str());
		div.set_class_name("positive");
		
		ca.append_child(&div).unwrap();
	});
	
	let doc = document.clone();
	let ca = clock_area.clone();
	let negative_button_handler = Closure::<dyn FnMut(_)>::new(move |e: PointerEvent| {
		let div = doc.create_element("div").unwrap();
		div.set_id(Uuid::new_v4().to_string().as_str());
		div.set_class_name("negative");
		
		ca.append_child(&div).unwrap();
	});	
	
	positive_button.add_event_listener_with_callback("click", positive_button_handler.as_ref().unchecked_ref())?;
	negative_button.add_event_listener_with_callback("click", negative_button_handler.as_ref().unchecked_ref())?;
	
	positive_button_handler.forget();
	negative_button_handler.forget();
	
	// The "add good clock" and "add bad clock" buttons just add an empty div to the page with an id that corresponds to a UUIDv4.
	// We'll need a loop to monitor for new divs, then create a `ProgressClock` to track it and initialise the canvas etc.
	
	Ok(())
}

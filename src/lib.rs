pub mod clock;

use crate::clock::ProgressClock;

use wasm_bindgen::prelude::*;
use web_sys::{ Document, HtmlInputElement, HtmlDivElement, PointerEvent };
use console_error_panic_hook;

use std::sync::Arc;
use std::sync::Mutex;

const POSITIVE_TICK_COLOR : &str = "#E0FFFF";
const NEGATIVE_TICK_COLOR : &str = "#A31F34";

fn add_clock(document: &Document, clock_area: &HtmlDivElement, clock: &ProgressClock) {
	let div = document.create_element("div").unwrap();
	div.set_id(&clock.get_id());
	clock_area.append_child(&div).unwrap();
	
	// Set the clock's title.
	let title = document.create_element("input").unwrap();
	let title : HtmlInputElement = title.dyn_into::<HtmlInputElement>().map_err(|_| ()).unwrap();
	title.set_class_name("clock-title");
	title.set_placeholder("Untitled Clock");
	if clock.get_name() != "" { 
		title.set_value(&clock.get_name());
	}
	match clock.is_positive() {
		true => title.style().set_property("color", POSITIVE_TICK_COLOR).unwrap(),
		false => title.style().set_property("color", NEGATIVE_TICK_COLOR).unwrap()
	};
	div.append_child(&title).unwrap();
	
	// TODO rest of this function
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
	let clocks: Arc<Mutex<Vec<ProgressClock>>> = Arc::new(Mutex::new(Vec::new()));

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
	let state = Arc::clone(&clocks);
	let doc = document.clone();
	let ca = clock_area.clone();
	let positive_button_handler = Closure::<dyn FnMut(_)>::new(move |_e: PointerEvent| {
		let mut clocks_handle = state.lock().unwrap();
		let new_clock = ProgressClock::new("", true);
		add_clock(&doc, &ca, &new_clock);
		clocks_handle.push(new_clock);
	});
	
	let state = Arc::clone(&clocks);
	let doc = document.clone();
	let ca = clock_area.clone();
	let negative_button_handler = Closure::<dyn FnMut(_)>::new(move |_e: PointerEvent| {
		let mut clocks_handle = state.lock().unwrap();
		let new_clock = ProgressClock::new("", false);
		add_clock(&doc, &ca, &new_clock);
		clocks_handle.push(new_clock);
	});	
	
	positive_button.add_event_listener_with_callback("click", positive_button_handler.as_ref().unchecked_ref())?;
	negative_button.add_event_listener_with_callback("click", negative_button_handler.as_ref().unchecked_ref())?;
	
	positive_button_handler.forget();
	negative_button_handler.forget();
	
	Ok(())
}

use wasm_bindgen::prelude::*;
use web_sys::{ HtmlInputElement, HtmlDivElement };

pub mod clock;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
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
	
	Ok(())
}

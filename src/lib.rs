pub mod clock;
pub mod render;
pub mod storage;

use crate::clock::ProgressClockMutex;
use crate::render::add_clock;
use crate::render::draw_clock;
use crate::storage::export_clocks;
use crate::storage::import_clocks;
use crate::storage::save_clocks;
use crate::storage::load_clocks;

use wasm_bindgen::prelude::*;
use web_sys::{ HtmlInputElement, Event, PointerEvent };
use console_error_panic_hook;

use std::sync::Arc;
use std::sync::Mutex;
use std::rc::Rc;
use std::cell::RefCell;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
	let clocks: Arc<Mutex<Vec<ProgressClockMutex>>> = Arc::new(Mutex::new(Vec::new()));

	// Get a handle to the document.
	let window = web_sys::window().expect("Could not find window!");
	let document = window.document().expect("Could not retrieve document from window!");
	
	// Get a handle to local storage.
	let storage = window.local_storage().expect("Could not retrieve local storage!").expect("Could not retrieve local storage!");
	
	// Retrieve elements from the page.
	let positive_button = document.get_element_by_id("add-positive").expect("Could not retrieve positive clock button!");
	let positive_button : HtmlInputElement = positive_button.dyn_into::<HtmlInputElement>().map_err(|_| ()).expect("Positive clock button has the wrong element type!");
	
	let negative_button = document.get_element_by_id("add-negative").expect("Could not retrieve negative clock button!");
	let negative_button : HtmlInputElement = negative_button.dyn_into::<HtmlInputElement>().map_err(|_| ()).expect("Negative clock button has the wrong element type!");

	let export_button = document.get_element_by_id("export").expect("Could not retrieve export button!");
	let export_button : HtmlInputElement = export_button.dyn_into::<HtmlInputElement>().map_err(|_| ()).expect("Export button has the wrong element type!");
	
	let import_button = document.get_element_by_id("import").expect("Could not retrieve import button!");
	let import_button : HtmlInputElement = import_button.dyn_into::<HtmlInputElement>().map_err(|_| ()).expect("Import button has the wrong element type!");
	
	let import_dialog = document.get_element_by_id("import-input").expect("Could not retrieve import input element!");
	let import_dialog : HtmlInputElement = import_dialog.dyn_into::<HtmlInputElement>().map_err(|_| ()).expect("Import input element has the wrong element type!");

	// Try to load existing clocks from storage, if available.
	load_clocks(&document, &storage, &clocks);

	// Set up event handlers for "add clock" buttons.
	let clocks_mx = Arc::clone(&clocks);
	let doc = document.clone();
	let handler = Closure::<dyn FnMut(_)>::new(move |_e: PointerEvent| {
		add_clock(None, &doc, &clocks_mx, true);
	});
	positive_button.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;
	handler.forget();
	
	let clocks_mx = Arc::clone(&clocks);
	let doc = document.clone();
	let handler = Closure::<dyn FnMut(_)>::new(move |_e: PointerEvent| {
		add_clock(None, &doc, &clocks_mx, false);
	});	
	negative_button.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;
	handler.forget();
	
	// Set up event handlers for import/export buttons.
	let clocks_mx = Arc::clone(&clocks);
	let doc = document.clone();
	let handler = Closure::<dyn FnMut(_)>::new(move |_e: PointerEvent| {
		export_clocks(&doc, &clocks_mx);
	});	
	export_button.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;
	handler.forget();

	let id = import_dialog.clone();
	let handler = Closure::<dyn FnMut(_)>::new(move |_e: PointerEvent| {
		id.click();
	});	
	import_button.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;
	handler.forget();
	
	let clocks_mx = Arc::clone(&clocks);
	let doc = document.clone();
	let handler = Closure::<dyn FnMut(_)>::new(move |_e: Event| {
		import_clocks(&doc, &clocks_mx);
	});	
	import_dialog.add_event_listener_with_callback("change", handler.as_ref().unchecked_ref())?;
	handler.forget();
	
	// Set up the main loop.
	let render_loop = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
	
	let w = window.clone();
	let rl = render_loop.clone();
	let clocks_mx = Arc::clone(&clocks);
	let doc = document.clone();
	let sto = storage.clone();
	*render_loop.borrow_mut() = Some(Closure::new(move || {
		// Draw the clocks.
		let clocks_handle = clocks_mx.lock().expect("Failed to obtain a lock on state mutex!");
		for clock in &*clocks_handle {
			draw_clock(&doc, clock, 0.0, 0.0);
		}
		
		// Save the clocks.
		std::mem::drop(clocks_handle);
		save_clocks(&sto, &clocks_mx);
		
		w.request_animation_frame(rl.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
	}));

	window.request_animation_frame(render_loop.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
	Ok(())
}

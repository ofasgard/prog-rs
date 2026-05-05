use crate::clock::ProgressClock;
use crate::clock::ProgressClockMutex;

use wasm_bindgen::prelude::*;
use web_sys::{ Document, HtmlInputElement, HtmlDivElement, HtmlCanvasElement, HtmlButtonElement, CanvasRenderingContext2d, Path2d, PointerEvent };

use std::f64;
use std::sync::Arc;
use std::sync::Mutex;

const CLOCK_RADIUS : u32 = 100;
const BACKGROUND_COLOR : &str = "#333333";
const CLOCK_COLOR : &str = "#444444";
const POSITIVE_TICK_COLOR : &str = "#E0FFFF";
const NEGATIVE_TICK_COLOR : &str = "#A31F34";

fn generate_wedge(degrees: f64) -> Path2d {
	let wedge = Path2d::new().expect("Failed to create Path2d!");
	let radius : f64 = CLOCK_RADIUS.into();
	
	wedge.arc(radius, radius, radius, 0.0, (degrees * f64::consts::PI) / 180.0).expect("Failed to draw an arc!");
	wedge.line_to(radius, radius);
	wedge.close_path();
	wedge
}

fn clock_exists(id: String, clocks_mx: Arc<Mutex<Vec<ProgressClockMutex>>>) -> bool {
	let clocks_handle = clocks_mx.lock().expect("Failed to obtain a lock on state mutex!");
	for clock_mx in &*clocks_handle {
		let clock = clock_mx.lock().expect("Failed to obtain a lock on a progress clock!");
		if clock.get_id() == id { return true; }
	}
	
	false
}

pub fn add_clock(clock: Option<ProgressClock>, document: &Document, clocks_mx: &Arc<Mutex<Vec<ProgressClockMutex>>>, positive: bool) {
	// Check that the clock doesn't already exist.
	if clock.is_some() {
		let id = clock.clone().unwrap().get_id();
		if clock_exists(id, Arc::clone(&clocks_mx)) { return; }
	}

	// Retrieve the clock area div.
	let clock_area = document.get_element_by_id("clock-area").expect("Could not retrieve clock area div!");
	let clock_area : HtmlDivElement = clock_area.dyn_into::<HtmlDivElement>().map_err(|_| ()).expect("Clock area div has the wrong element type!");

	// Create an object for the new clock.
	let mut clocks_handle = clocks_mx.lock().expect("Failed to obtain a lock on state mutex!");
	let new_clock = match clock {
		Some(provided) => Arc::new(Mutex::new(provided)),
		None => Arc::new(Mutex::new(ProgressClock::new("", positive)))
	};
	
	let clock_mx = new_clock.clone();
	let clock = clock_mx.lock().expect("Failed to obtain a lock on a progress clock!");
	
	clocks_handle.push(new_clock);

	// Add a div for the new clock.
	let div = document.create_element("div").expect("Failed to create a div element!");
	div.set_class_name("clock-container");
	div.set_id(&clock.get_id());
	clock_area.append_child(&div).expect("Failed to add a div element to the clock area div!");
	
	// Set the clock's title.
	let title = document.create_element("input").expect("Failed to create an input element!");
	let title : HtmlInputElement = title.dyn_into::<HtmlInputElement>().map_err(|_| ()).expect("Created input element has the wrong element type!");
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
	
	// Add an event handler so that changes to the title update the underlying object.
	let handler_clock = Arc::clone(&clock_mx);
	let handler_title = title.clone();
	let handler = Closure::<dyn FnMut()>::new(move || {
		handler_clock.lock().unwrap().set_name(&handler_title.value());
	});
	
	title.add_event_listener_with_callback("change", handler.as_ref().unchecked_ref()).unwrap();
	handler.forget();
	
	// Create a canvas to draw the clock on.
	let canvas = document.create_element("canvas").unwrap();
	let canvas : HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().map_err(|_| ()).unwrap();
	canvas.set_class_name("clock-canvas");
	canvas.set_width(CLOCK_RADIUS * 2);
	canvas.set_height(CLOCK_RADIUS * 2);
	div.append_child(&canvas).unwrap();
	
	// Create the buttons to enlarge/reduce/remove the clock.
	let button_div = document.create_element("div").unwrap();
	button_div.set_class_name("clock-buttons");
	div.append_child(&button_div).unwrap();
	
	let enlarge = document.create_element("button").unwrap();
	let enlarge : HtmlButtonElement = enlarge.dyn_into::<HtmlButtonElement>().map_err(|_| ()).unwrap();
	enlarge.class_list().add_1("button").unwrap();
	enlarge.class_list().add_1("clock-button").unwrap();
	enlarge.set_inner_text("↟");
	button_div.append_child(&enlarge).unwrap();
	
	let remove = document.create_element("button").unwrap();
	let remove : HtmlButtonElement = remove.dyn_into::<HtmlButtonElement>().map_err(|_| ()).unwrap();
	remove.class_list().add_1("button").unwrap();
	remove.class_list().add_1("clock-button").unwrap();
	remove.set_inner_text("🗑");
	button_div.append_child(&remove).unwrap();
	
	let reduce = document.create_element("button").unwrap();
	let reduce : HtmlButtonElement = reduce.dyn_into::<HtmlButtonElement>().map_err(|_| ()).unwrap();
	reduce.class_list().add_1("button").unwrap();
	reduce.class_list().add_1("clock-button").unwrap();
	reduce.set_inner_text("↡");
	button_div.append_child(&reduce).unwrap();
	
	// Event handler for enlarge button.
	let handler_clock = Arc::clone(&clock_mx);
	let handler = Closure::<dyn FnMut()>::new(move || {
		handler_clock.lock().unwrap().enlarge();
	});
	
	enlarge.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref()).unwrap();
	handler.forget();
	
	// Event handler for remove button.
	let doc = document.clone();
	let handler_clocks = Arc::clone(&clocks_mx);
	let handler_clock = Arc::clone(&clock_mx);
	let handler = Closure::<dyn FnMut()>::new(move || {
		remove_clock(&doc, &handler_clocks, &handler_clock);
	});
	
	remove.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref()).unwrap();
	handler.forget();
	
	// Event handler for reduce button.
	let handler_clock = Arc::clone(&clock_mx);
	let handler = Closure::<dyn FnMut()>::new(move || {
		handler_clock.lock().unwrap().reduce();
	});
	
	reduce.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref()).unwrap();
	handler.forget();
	
	// Event handler for processing clicks.
	let handler_clock = Arc::clone(&clock_mx);
	let doc = document.clone();
	let canv = canvas.clone();
	let handler = Closure::<dyn FnMut(_)>::new(move |e: PointerEvent| {
		let bounding_rect = canv.get_bounding_client_rect();
		
		let scale_x = (canv.width() as f64) / bounding_rect.width();
		let scale_y = (canv.height() as f64) / bounding_rect.height();
		
		let canvas_x = (e.client_x() as f64 - bounding_rect.left()) * scale_x;
		let canvas_y = (e.client_y() as f64 - bounding_rect.top()) * scale_y;
		
		if let Some(wedge) = check_tick(&doc, &handler_clock, canvas_x, canvas_y) {
			handler_clock.lock().unwrap().process_click(wedge);
		}
	});
	
	canvas.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref()).unwrap();
	handler.forget();
}

pub fn remove_clock(document: &Document, clocks_mx: &Arc<Mutex<Vec<ProgressClockMutex>>>, clock_mx: &ProgressClockMutex) {
	let clock = clock_mx.lock().expect("Failed to obtain a lock on a progress clock!");
	let id = clock.get_id();
	std::mem::drop(clock); // must explicitly drop to avoid recursive mutex
	
	let element = document.get_element_by_id(&id).unwrap();
	element.remove();
	
	let mut clocks_handle = clocks_mx.lock().expect("Failed to obtain a lock on state mutex!");
	clocks_handle.retain(|i| {
		let current_clock = i.lock().unwrap();
		!(current_clock.get_id() == id)
	});
}

pub fn draw_clock(document: &Document, clock_mx: &ProgressClockMutex, x: f64, y: f64) {
	// Retrieve the div that corresponds to this clock.
	let clock = clock_mx.lock().expect("Failed to obtain a lock on a progress clock!");
	let id = clock.get_id();
	let size = clock.get_size();
	let ticks = clock.get_ticks();
	
	let clock_div = document.get_element_by_id(&id).unwrap();
	let clock_div : HtmlDivElement = clock_div.dyn_into::<HtmlDivElement>().map_err(|_| ()).unwrap();
	
	// Retrieve the canvas and initialise a drawing context.
	let canvas = clock_div.children().item(1).unwrap();
	let canvas : HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().map_err(|_| ()).unwrap();
	
	let ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
	ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
	
	// Move our context to the correct position.
	ctx.translate(x, y).unwrap();
	
	// Dynamically create a wedge of the correct angle.
	let degrees : f64 = 360.0 / (size as f64);
	let wedge : Path2d = generate_wedge(degrees);
	
	// Prepare the context.
	ctx.set_stroke_style_str(BACKGROUND_COLOR);
	ctx.set_line_width(10.0);
	
	// Initial rotation to orient the clock correctly.
	let radius : f64 = CLOCK_RADIUS.into();
	ctx.translate(radius, radius).unwrap();
	ctx.rotate((270.0 * f64::consts::PI) / 180.0).unwrap();
	ctx.translate(-radius, -radius).unwrap();
	
	// Iterate through all wedges and draw them in the correct position.
	for i in 0..size {
		match ticks > i {
			true => {
				let color = match clock.is_positive() { true => POSITIVE_TICK_COLOR, false => NEGATIVE_TICK_COLOR };
				ctx.set_fill_style_str(color);
			}
			false => ctx.set_fill_style_str(CLOCK_COLOR)
		}
		
		ctx.fill_with_path_2d(&wedge);
		ctx.stroke_with_path(&wedge);
		
		ctx.translate(radius, radius).unwrap();
		ctx.rotate((degrees * f64::consts::PI) / 180.0).unwrap();
		ctx.translate(-radius, -radius).unwrap();
	}
	
	ctx.reset_transform().unwrap();
}

pub fn check_tick(document: &Document, clock_mx: &ProgressClockMutex, click_x: f64, click_y: f64) -> Option<i32> {
	let clock = clock_mx.lock().expect("Failed to obtain a lock on a progress clock!");
	let id = clock.get_id();
	let size = clock.get_size();

	let clock_div = document.get_element_by_id(&id).unwrap();
	let clock_div : HtmlDivElement = clock_div.dyn_into::<HtmlDivElement>().map_err(|_| ()).unwrap();

	// Retrieve the canvas and initialise a drawing context.
	let canvas = clock_div.children().item(1).unwrap();
	let canvas : HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().map_err(|_| ()).unwrap();
	
	let ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
	ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
	
	// Dynamically create a wedge of the correct angle.
	let degrees : f64 = 360.0 / (size as f64);
	let wedge : Path2d = generate_wedge(degrees);
	
	// Initial rotation to orient the clock correctly.
	let radius : f64 = CLOCK_RADIUS.into();
	ctx.translate(radius, radius).unwrap();
	ctx.rotate((270.0 * f64::consts::PI) / 180.0).unwrap();
	ctx.translate(-radius, -radius).unwrap();
	
	// Iterate through all wedges and check whether the given point is inside each of them.
	for i in 0..size {
		if ctx.is_point_in_path_with_path_2d_and_f64(&wedge, click_x, click_y) {
			ctx.reset_transform().unwrap();
			return Some(i+1);
		}
		
		ctx.translate(radius, radius).unwrap();
		ctx.rotate((degrees * f64::consts::PI) / 180.0).unwrap();
		ctx.translate(-radius, -radius).unwrap();
	}
	
	// If not in any wedge, return None.
	ctx.reset_transform().unwrap();
	None
}

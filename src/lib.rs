use wasm_bindgen::prelude::*;

pub mod clock;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
	Ok(())
}

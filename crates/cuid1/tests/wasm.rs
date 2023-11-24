use cuid::cuid;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn example() {
    let id = cuid().unwrap();
    assert!(id.len() == 25);
}

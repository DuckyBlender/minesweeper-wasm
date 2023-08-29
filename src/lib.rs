mod minesweeper;
mod random;

use std::cell::RefCell;

use minesweeper::*;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

thread_local! {
  static MINESWEEPER: RefCell<Minesweeper>
    = RefCell::new(Minesweeper::new(20, 20, 40)); // medium difficulty by default
}

#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
    MINESWEEPER.with(|ms| ms.borrow().to_string())
}

#[wasm_bindgen(js_name = revealField)]
pub fn reveal_field(x: usize, y: usize) {
    MINESWEEPER.with(|ms| {
        ms.borrow_mut().reveal((x, y));
    });
}

#[wasm_bindgen(js_name = flagField)]
pub fn flag_field(x: usize, y: usize) {
    MINESWEEPER.with(|ms| {
        ms.borrow_mut().flag((x, y));
    });
}

#[wasm_bindgen(js_name = reset)]
pub fn reset() {
    MINESWEEPER.with(|ms| {
        ms.borrow_mut().reset();
    });
}

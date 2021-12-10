use serde::Serialize;
use wasm_bindgen::prelude::*;

use libwhen::TimeAtLocation;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize)]
pub struct ParseResult {
    is_relative: bool,
    locations: Vec<TimeAtLocation>,
    error: Option<String>,
}

fn handle_expr(input: &str) -> Result<(Vec<TimeAtLocation>, bool), String> {
    let expr = libwhen::InputExpr::parse(&input).map_err(|x| x.to_string())?;
    Ok(expr
        .process()
        .map(|x| (x, expr.is_relative()))
        .map_err(|x| x.to_string())?)
}

#[wasm_bindgen]
pub fn parse_expr(input: String) -> String {
    let (locations, is_relative, error) = match handle_expr(&input) {
        Ok((locations, is_relative)) => (locations, is_relative, None),
        Err(err) => (Vec::new(), false, Some(err.to_string())),
    };
    serde_json::to_string(&ParseResult {
        is_relative,
        locations,
        error,
    })
    .unwrap()
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

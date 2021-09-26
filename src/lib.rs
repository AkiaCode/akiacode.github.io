// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model;

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {}

// `update` describes how to handle each `Msg`.
fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(_: &Model) -> Node<Msg> {
    div![
        "This is a counter: test",
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}

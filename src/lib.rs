// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use rand::Rng;
use seed::{*, prelude::*};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        url: None
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    url: Option<String>
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Fetch,
    Received(Option<String>),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Fetch => {
            orders.skip(); // No need to rerender
            orders.perform_cmd(async {
                if rand::thread_rng().gen::<bool>() {
                    let response = fetch("https://nekos.life/api/v2/img/meow").await.expect("HTTP request failed");

                    if let Ok(resp_json) = response.check_status() {
                        if let Ok(json) = resp_json.json::<serde_json::Value>().await {
                            if let Some(url) = json.get("url") {
                                Msg::Received(Some(url.to_string()))
                            } else {
                                Msg::Received(None)
                            }
                        } else {
                            Msg::Received(None)
                        }
                    } else {
                        Msg::Received(None)
                    }
                } else {
                    Msg::Received(None)
                }
            });
        }
        Msg::Received(url) => {
            model.url = url;
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {

    div![
        style! {
            St::BackgroundImage => if let Some(url) = &model.url {
                format!("url({})", url)
            } else {
                "none".to_string()
            },
            St::Position => "fixed",
            St::Top => "0",
            St::Left => "0",
            St::Display => "flex",
            St::JustifyContent => "center",
            St::AlignItems => "center",
            St::Width => "100%",
            St::Height => "100%",
            St::FlexDirection => "column",
            St::FontFamily => "Helvetica, Arial, sans-serif",
        },
        div!{
            style!{
                St::Width => px(350),
                St::Height => px(350),
            },
            h1! {
                "👩‍💻 Welcome, I`m ",
                a! {
                    style! {
                        St::Color => "#f0a3d6",
                        St::TextDecoration => "none",
                    },
                    attrs! {
                        At::Href => "https://github.com/akiacode",
                        At::Target => "_blank",
                    },
                    "AkiaCode"
                }
            },
            h3!{
                style!{
                    St::MarginLeft => px(8),
                },
                "My Projects",
           },
           ul! {
               style! {
                    St::ListStyleType => "circle",
                    St::Color => "#f0a3d6",
                },
                li!["Harmony"],
                li!["Other..."],
            },
            h3 ! {
                style!{
                    St::MarginLeft => px(8),
                },
                "My Skills",
            },
            ul! {
                style! {
                    St::ListStyleType => "circle",
                    St::Color => "#f0a3d6",
                },
                li!["Rust"],
                li!["JavaScript"],
                li!["TypeScript"],
                li!["Python"],
                li!["C"],
                li!["Other..."],
            },
            h5! {
                style! {
                    St::MarginLeft => px(8),
                },
                "Made with ❤️ by ",
                a! {
                    style! {
                        St::Color => "#f0a3d6",
                        St::TextDecoration => "none",
                    },
                    attrs! {
                        At::Href => "https://github.com/akiacode",
                        At::Target => "_blank",
                    },
                    "AkiaCode"
                }
            }
        }
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

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{hotreload::FreyaCtx, prelude::*};

use dioxus::{hooks::*, prelude::GlobalAttributes};

use crate::prompt::Prompt;

mod prompt;

fn main() {
    dioxus_hot_reload::hot_reload_init!(Config::<FreyaCtx>::default());

    launch(app);
}


pub type History = im_rc::HashMap<u32, HistoryItem>;

#[derive(Debug, PartialEq)]
pub struct HistoryItem {
    pub id: u32,
    pub expression: String,
    pub result: fend_core::FendResult,
}

fn app(cx: Scope) -> Element {

    let history = use_ref(cx, History::default);

    render!(

        ThemeProvider {
            theme: DARK_THEME,
            rect {
                background: "rgb(15, 15, 15)",
                width: "100%",
                height: "auto",
                ScrollView {
                    show_scrollbar: true,
                    direction: "vertical",
                    rect {
                        padding: "24 50 0 50",
                        history.read().iter().map(|(k, v)| rsx!(label {
                            key: "{k}",
                            height: "80",
                            color: "white",
                            "Number {k}: {v.result.get_main_result()}"
                        }
                        ))
                    }
                    Prompt {}
                } }
        },
    )
}

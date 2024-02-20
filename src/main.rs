#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

use crate::prompt::Prompt;

mod prompt;
mod timeout;

fn main() {
    launch(app);
}

pub type History = Vec<HistoryItem>;

#[derive(Debug, PartialEq)]
pub struct HistoryItem {
    // pub id: u32,
    pub expression: String,
    pub result: fend_core::FendResult,
}

fn app(cx: Scope) -> Element {
    let history = use_ref(cx, History::default);

    let context = use_ref(cx, || fend_core::Context::new());

    let execute_prompt = move |data: prompt::SubmitData| {
        let interrupt = timeout::TimeoutInterrupt::new_with_timeout(640 as u128);
        let res = fend_core::evaluate_with_interrupt(
            &data.prompt,
            &mut context.read(),
            &interrupt,
        );
        if let Ok(res) = res {
            history.with_mut(|h| {
                h.push(HistoryItem {
                    expression: data.prompt,
                    result: res,
                })
            })
        } else if let Err(e) = res {
            println!("{e}")
        }
    };

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
                        history.read().iter().enumerate().map(|(k, v)| rsx!(
                            rect {
                                key: "{k}",
                                label {
                                    color: "white",
                                    "{v.expression}"
                                },
                                label {
                                    color: "white",
                                    "= {v.result.get_main_result()}"
                                }
                            }
                        ))
                    },
                    Prompt {
                        context: context,
                        on_submit: execute_prompt,
                    }
                } }
        },
    )
}

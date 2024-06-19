#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_hooks::use_signal;
use dioxus_signals::{Readable, Writable};
use freya::prelude::*;

use crate::prompt::Prompt;

mod exchange_rates;
mod file_paths;
mod prompt;
mod timeout;

const ICON: &[u8] = include_bytes!("../assets/icon.png");
fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_title("FendApp")
            .with_icon(LaunchConfig::load_icon(ICON))
            .build(),
    )
}

pub type History = Vec<HistoryItem>;

#[derive(Debug, PartialEq)]
pub struct HistoryItem {
    // pub id: u32,
    pub expression: String,
    pub result: fend_core::FendResult,
}

fn app() -> Element {
    let mut history = use_signal(History::default);

    let exchange_rates = use_signal(|| exchange_rates::ExchangeRateHandler {
        enable_internet_access: true,
        source: exchange_rates::ExchangeRateSource::EuropeanUnion,
    });

    let mut context = use_signal(fend_core::Context::new);
    context.with_mut(|c| {
        let rates = exchange_rates.read();
        c.set_exchange_rate_handler_v1(*rates);
        c.set_random_u32_fn(random_u32);
    });

    let execute_prompt = move |data: prompt::SubmitData| {
        let interrupt = timeout::TimeoutInterrupt::new_with_timeout(640_u128);
        let res =
            context.with_mut(|c| fend_core::evaluate_with_interrupt(&data.prompt, c, &interrupt));
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

    rsx!(
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
                        {
                            history.read().iter().enumerate().map(|(k, v)| rsx!{
                            rect {
                                key: "{k}",
                                label {
                                    color: "white",
                                    "{v.expression}"
                                },
                                label {
                                    color: "white",
                                    "= {v.result.get_main_result()}"
                                },
                            },
                        })
                    },
                    },
                    Prompt {
                        context: context,
                        on_submit: execute_prompt,
                    },
                } }
        },
    )
}

fn random_u32() -> u32 {
    rand::random()
}

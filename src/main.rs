#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_hooks::{use_effect, use_signal};
use dioxus_sdk::clipboard::use_clipboard;
use dioxus_signals::{Readable, Writable};
use fend_core::FendResult;
use freya::prelude::*;

use crate::prompt::Prompt;

mod exchange_rates;
mod file_paths;
mod prompt;
mod timeout;

const ICON: &[u8] = include_bytes!("../assets/icon.png");

fn main() {
    let _guard = sentry::init((
        "https://a1c9c9b6cf52b493705e9c73b3310292@relay.ellis.link/4510506397007952",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            auto_session_tracking: true,
            session_mode: sentry::SessionMode::Application,
            ..Default::default()
        },
    ));

    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_title("FendApp")
            .with_icon(LaunchConfig::load_icon(ICON))
            .with_size(500.0, 600.0),
    )
}

pub type History = Vec<HistoryItem>;

#[derive(Debug, PartialEq, Clone)]
pub struct HistoryItem {
    // pub id: u32,
    pub expression: String,
    pub result: HistoryResult,
}
impl From<FendResult> for HistoryResult {
    fn from(result: FendResult) -> Self {
        let mut plain_result = String::new();
        let mut spans = vec![];
        for span in result.get_main_result_spans() {
            plain_result.push_str(span.string());
            spans.push((
                plain_result.len() - span.string().len()..plain_result.len(),
                span.kind(),
            ));
        }
        Self {
            plain_result,
            spans,
            is_empty: result.output_is_empty(),
            attrs: Attrs {
                trailing_newline: result.has_trailing_newline(),
            },
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HistoryResult {
    plain_result: String,
    spans: Vec<(std::ops::Range<usize>, fend_core::SpanKind)>,
    is_empty: bool, // is this the () type
    attrs: Attrs,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpanRef<'a> {
    string: &'a str,
    kind: fend_core::SpanKind,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct Attrs {
    // pub(crate) debug: bool,
    // pub(crate) show_approx: bool,
    // pub(crate) plain_number: bool,
    pub(crate) trailing_newline: bool,
}
impl Default for Attrs {
    fn default() -> Self {
        Self {
            // debug: false,
            // show_approx: true,
            // plain_number: false,
            trailing_newline: true,
        }
    }
}

impl HistoryResult {
    /// This retrieves the main result of the computation.
    #[must_use]
    pub fn get_main_result(&self) -> &str {
        self.plain_result.as_str()
    }

    /// This retrieves the main result as a list of spans, which is useful
    /// for colored output.
    pub fn get_main_result_spans(&self) -> impl Iterator<Item = SpanRef<'_>> {
        self.spans.iter().map(|(span, kind)| SpanRef {
            string: &self.plain_result[span.clone()],
            kind: *kind,
        })
    }

    /// Returns whether or not the result is the `()` type. It can sometimes
    /// be useful to hide these values.
    #[must_use]
    pub fn is_unit_type(&self) -> bool {
        self.is_empty
    }

    /// Returns whether or not the result should be outputted with a
    /// trailing newline. This is controlled by the `@no_trailing_newline`
    /// attribute.
    #[must_use]
    pub fn has_trailing_newline(&self) -> bool {
        self.attrs.trailing_newline
    }
}

fn get_theme(preferred_theme: PreferredTheme) -> Theme {
    match preferred_theme {
        PreferredTheme::Dark => DARK_THEME,
        PreferredTheme::Light => LIGHT_THEME,
    }
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
        c.set_exchange_rate_handler_v2(*rates);
        c.set_random_u32_fn(random_u32);
    });

    let mut error = use_signal(|| Option::None);

    // Theme handling
    let preferred_theme = use_preferred_theme();
    let mut current_theme = use_init_theme(|| get_theme(*preferred_theme.peek()));

    use_effect(move || {
        let theme = get_theme(preferred_theme());
        if theme != *current_theme.peek() {
            current_theme.set(theme);
        }
    });

    let is_dark = current_theme.read().name == "dark";
    let background_color = if is_dark {
        "rgb(15, 15, 15)"
    } else {
        "rgb(255, 255, 255)"
    };
    let text_color = if is_dark { "#d0d0d0" } else { "#2f2f2f" };
    let result_color = if is_dark { "white" } else { "black" };
    let input_background = if is_dark {
        "rgb(25, 25, 25)"
    } else {
        "rgb(245, 245, 245)"
    };

    let execute_prompt = move |mut data: prompt::SubmitData| {
        let interrupt = timeout::TimeoutInterrupt::new_with_timeout(640_u128);
        let res = context.with_mut(|c| {
            fend_core::evaluate_with_interrupt(&data.prompt.read().to_string(), c, &interrupt)
        });
        if let Ok(res) = res {
            history.with_mut(|h| {
                h.push(HistoryItem {
                    expression: data.prompt.read().to_string(),
                    result: res.into(),
                })
            });
            data.prompt.set(String::new());
            error.set(None);
        } else if let Err(e) = res {
            println!("{e}");
            error.set(Some(e));
        }
    };

    let mut clipboard = use_clipboard();
    let platform = use_platform();

    rsx!(
        ThemeProvider {
            theme: current_theme.read().clone(),
            rect {
                background: "{background_color}",
                width: "100%",
                height: "auto",
                ScrollView {
                    show_scrollbar: true,
                    direction: "vertical",
                    rect {
                        padding: "24 50 0 50",
                        {
                            history.read().clone().into_iter().enumerate().map(|(k, v)| rsx!{
                            rect {
                                key: "{k}",
                                label {
                                    color: "{text_color}",
                                    onmouseenter: move |_| {
                                        platform.set_cursor(CursorIcon::Pointer);
                                    },
                                    onmouseleave: move |_| {
                                        platform.set_cursor(CursorIcon::Default);
                                    },
                                    onclick: move |_| {
                                        let _ = clipboard.set(v.expression.clone());
                                        // TODO: Notify user that the expression was copied
                                    },
                                    "{v.expression}"
                                },
                                label {
                                    color: "{result_color}",

                                    onmouseenter: move |_| {
                                        platform.set_cursor(CursorIcon::Pointer);
                                    },
                                    onmouseleave: move |_| {
                                        platform.set_cursor(CursorIcon::Default);
                                    },
                                    onclick: move |_| {
                                        let _ = clipboard.set(v.result.get_main_result().to_string());
                                        // TODO: Notify user that the expression was copied
                                    },
                                    "= {v.result.get_main_result()}"
                                },
                            },
                        })
                    },
                    },
                    Prompt {
                        context: context,
                        on_submit: execute_prompt,
                        error,
                        preview_color: result_color.to_string(),
                        error_color: "red".to_string(),
                        input_background: input_background.to_string(),
                    },
                } }
        },
    )
}

fn random_u32() -> u32 {
    rand::random()
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn integration() {
        use freya_testing::prelude::*;
        let mut utils = launch_test(super::app);

        let rect = utils.root().get(0).get(0).get(0).get(0);

        utils.wait_for_update().await;
        assert_eq!(rect.children_ids().len(), 2);

        let input = rect.get(1).get(0);
        for (text, key) in [
            ("1", Code::Digit1),
            ("+", Code::NumpadAdd),
            ("3", Code::Digit3),
        ] {
            utils.push_event(TestEvent::Keyboard {
                name: EventName::KeyDown,
                key: Key::Character(text.into()),
                code: key,
                modifiers: Modifiers::default(),
            });
            utils.wait_for_update().await;
        }

        input.get_by_text("1+3").expect("input worked");
        utils.wait_for_update().await;
        let preview_label = rect.get(1).get(1);
        preview_label.get_by_text("4").expect("preview worked");
    }
}

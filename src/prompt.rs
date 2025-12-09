use dioxus_hooks::{use_memo, use_signal};
use dioxus_signals::{Readable, Signal, Writable};
use fend_core::Context;
use freya::prelude::*;

use crate::timeout::TimeoutInterrupt;

// #[derive(Props, Clone, PartialEq)]
// pub(crate) struct PromptProps {
//

// }

#[derive(Debug)]
pub(crate) struct SubmitData {
    pub prompt: Signal<String>,
}

#[allow(non_snake_case)]
#[component]
pub(crate) fn Prompt(
    on_submit: EventHandler<SubmitData>,
    mut context: Signal<Context>,
    error: Signal<Option<String>>,
    preview_color: String,
    error_color: String,
    input_background: String,
) -> Element {
    let mut prompt = use_signal(String::new);

    let preview = use_memo(move || {
        let interrupt = TimeoutInterrupt::new_with_timeout(32_u128);
        context
            .with_mut(|c| fend_core::evaluate_preview_with_interrupt(&prompt.read(), c, &interrupt))
    });

    rsx!(
        rect {
            padding: "0 24 24 24",
            onkeydown: move |e| {
                if e.data.key == keyboard::Key::Enter {
                    on_submit.call(SubmitData {
                        prompt
                    });
                }
            },
            Input {
                mode: InputMode::Shown,
                value: prompt.read().clone(),
                auto_focus: true,
                // width: "100%",
                onchange: move |e| {
                    prompt.set(e)
                },
                width: "100%",theme: InputThemeWith {
                    margin: Some("8 0".into()),
                    background: Some(input_background.clone().into()),
                    font_theme: Some(FontThemeWith {
                        color: Some(preview_color.clone().into()),
                    }),
                    ..Default::default()
                }
            },
            if !preview.read().output_is_empty() {
                label {
                    color: "{preview_color}",
                    "{preview.read().get_main_result()}"
                }
            }
            match error.read().as_ref() {
                Some(e) => rsx!{
                    label {
                        color: "{error_color}",
                        "{e}"
                    }
                },
                None => rsx!{},
            }
        },
    )
}

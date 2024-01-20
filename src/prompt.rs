use fend_core::Context;
use freya::prelude::*;

use crate::timeout::TimeoutInterrupt;

#[derive(Props)]
pub(crate) struct PromptProps<'a> {
    context: &'a UseRef<Context>,
    on_submit: EventHandler<'a, SubmitData>,
}

#[derive(Debug)]
pub(crate) struct SubmitData {
    pub prompt: String,
}

#[allow(non_snake_case)]
pub(crate) fn Prompt<'a>(cx: Scope<'a, PromptProps<'a>>) -> Element {
    let prompt = use_ref(cx, || String::new());

    let preview = use_memo(cx, (prompt,), |(prompt,)| {
        let interrupt = TimeoutInterrupt::new_with_timeout(32 as u128);
        fend_core::evaluate_preview_with_interrupt(
            &*prompt.read(),
            &mut cx.props.context.read().clone(),
            &interrupt,
        )
    });

    render!(
        rect {
            padding: "0 24 24 24",
            onkeydown: |e| {
                if e.data.key == keyboard::Key::Enter {
                    cx.props.on_submit.call(SubmitData {
                        prompt: prompt.read().to_string()
                    });
                    prompt.set(String::new());
                }
            },
            Input {
                mode: InputMode::Shown,
                value: prompt.read().clone(),
                // width: "100%",
                onchange: |e| {
                    prompt.set(e)
                },
                theme: InputThemeWith {
                    width: Some("100%".into()),
                    ..Default::default()
                }

            },
            label {
                color: "white",
                "{preview.get_main_result()}"
            }
        },
    )
}

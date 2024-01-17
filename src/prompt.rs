use freya::prelude::*;

use dioxus::hooks::*;
struct TimeoutInterrupt {
    start: std::time::Instant,
    timeout: u128,
}

impl TimeoutInterrupt {
    fn new_with_timeout(timeout: u128) -> Self {
        Self {
            start: std::time::Instant::now(),
            timeout,
        }
    }
}

impl fend_core::Interrupt for TimeoutInterrupt {
    fn should_interrupt(&self) -> bool {
        std::time::Instant::now()
            .duration_since(self.start)
            .as_millis()
            > self.timeout
    }
}

#[allow(non_snake_case)]
pub fn Prompt(cx: Scope) -> Element {
    let prompt = use_ref(cx, || String::new());

    let context = use_ref(cx, || fend_core::Context::new());

    let interrupt = TimeoutInterrupt::new_with_timeout(10 as u128);
    let preview = fend_core::evaluate_preview_with_interrupt(
        &*prompt.read(),
        &mut context.read().clone(),
        &interrupt,
    );

    render!(
            rect {
                padding: "0 24 24 24",
            Input {
                mode: InputMode::Shown,
                value: prompt.read().clone(),
                // width: "100%",
                onchange: |e| {
                    prompt.set(e)
                },
                // onkeydown: |e| println!("Event: {e:?}")
            },

            label {
                color: "white",
                "{preview.get_main_result()}"
            }
            


    },
    )
}

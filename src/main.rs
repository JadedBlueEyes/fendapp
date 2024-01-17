#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use freya::{hotreload::FreyaCtx, prelude::*, elements::rect};
use dioxus::hooks::*;
fn main() {
    dioxus_hot_reload::hot_reload_init!(Config::<FreyaCtx>::default());

    launch(app);
}


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
        std::time::Instant::now().duration_since(self.start).as_millis() > self.timeout
    }
}


fn app(cx: Scope) -> Element {
    
    let prompt = use_ref(cx, || String::new());
    
    let context = use_ref(cx, || fend_core::Context::new());
    
    let interrupt = TimeoutInterrupt::new_with_timeout(10 as u128);
    let preview = fend_core::evaluate_preview_with_interrupt(&*prompt.read(), &mut context.read().clone(), &interrupt);
    

    render!(
        
        ThemeProvider {
            theme: DARK_THEME,
        rect {
            overflow: "clip",
            background: "rgb(15, 15, 15)",
            padding: "50",
            width: "100%",
            height: "100%",
            Input {
                mode: InputMode::Shown,
                value: prompt.read().clone(),
                onchange: |e| {
                    prompt.set(e)
                },
            },
            rect {
                
            label {
                // font_size: "75",
                // font_weight: "bold",
                color: "white",
                "{preview.get_main_result()}"
            }
            }
        }
        
    },
    )
}

#[allow(non_snake_case)]
fn Comp(cx: Scope) -> Element {
    render!(rect {
        width: "50%",
        height: "100%",
        background: "yellow"
    })
}

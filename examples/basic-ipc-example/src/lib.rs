use std::cell::RefCell;

use openharmony_ability::OpenHarmonyApp;
use openharmony_ability_derive::ability;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    platform::ohos::EventLoopBuilderExtOpenHarmony,
    window::WindowBuilder,
};
use wry::{http::Request, WebViewBuilder};

const INDEX_HTML: &str = include_str!("./index.html");

thread_local! {
  static WEBVIEW: RefCell<Option<wry::WebView>> = RefCell::new(None);
}

enum UserEvent {
    Hello(String),
}

#[ability(webview)]
fn openharmony(app: OpenHarmonyApp) {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event()
        .with_openharmony_app(app.clone())
        .build();
    let window = WindowBuilder::new()
        .with_title("Hello World")
        .build(&event_loop)
        .unwrap();

    let proxy = event_loop.create_proxy();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("Wry has started!");
            }
            Event::Resumed => {
                let e = proxy.clone();
                let handler = move |req: Request<String>| {
                    let body = req.body();
                    match body.as_str() {
                        _ if body.starts_with("hello:") => {
                            let color = body.replace("hello:", "");
                            let _ = e.send_event(UserEvent::Hello(color));
                        }
                        _ => {}
                    }
                };
                let webview = WebViewBuilder::new()
                    .with_html(INDEX_HTML)
                    .with_ipc_handler(handler)
                    .build(&window)
                    .map_err(|e| {
                        let s = e.to_string();
                        println!("Failed to build webview: {}", s);
                    })
                    .unwrap();

                WEBVIEW.with(|w| {
                    *w.borrow_mut() = Some(webview);
                });
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::UserEvent(e) => match e {
                UserEvent::Hello(color) => {
                    let _ = WEBVIEW.with(|w| {
                      let _ = w.borrow_mut().as_ref().unwrap().evaluate_script("console.log('Hello from Rust!');");
                        let _ = w.borrow_mut().as_ref().unwrap().evaluate_script(&format!(
                            "document.body.style.backgroundColor = '{}'",
                            color
                        ));
                    });
                }
            },
            _ => (),
        }
    });
}

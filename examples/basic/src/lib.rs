use dpi::{LogicalPosition, LogicalSize};
use openharmony_ability::OpenHarmonyApp;
use openharmony_ability_derive::ability;
use winit::platform::ohos::EventLoopBuilderExtOpenHarmony;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};
use wry::{Rect, WebViewBuilder};

#[derive(Default)]
struct State {
    window: Option<Window>,
    webview: Option<wry::WebView>,
}

impl ApplicationHandler for State {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes();
        // attributes.inner_size = Some(LogicalSize::new(800, 800).into());
        let window = event_loop.create_window(attributes).unwrap();

        let webview = WebViewBuilder::new()
            .with_url("https://tauri.app")
            .build_as_child(&window)
            .unwrap();

        self.window = Some(window);
        self.webview = Some(webview);
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) => {
                let window = self.window.as_ref().unwrap();
                let webview = self.webview.as_ref().unwrap();

                let size = size.to_logical::<u32>(window.scale_factor());
                webview
                    .set_bounds(Rect {
                        position: LogicalPosition::new(0, 0).into(),
                        size: LogicalSize::new(size.width, size.height).into(),
                    })
                    .unwrap();
            }
            WindowEvent::CloseRequested => {
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

#[ability(webview)]
pub fn openharmony(app: OpenHarmonyApp) {
    let event_loop = EventLoop::with_user_event()
        .with_openharmony_app(app.clone())
        .build()
        .unwrap();
    let mut state = State::default();
    let state_app = Box::leak(Box::new(state));
    event_loop.run_app(state_app).unwrap();
}

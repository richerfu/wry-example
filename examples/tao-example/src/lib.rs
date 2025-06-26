// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use openharmony_ability::OpenHarmonyApp;
use openharmony_ability_derive::ability;
use tao::{
    event::Event, event_loop::EventLoopBuilder, platform::ohos::EventLoopBuilderExtOpenHarmony,
    window::WindowBuilder,
};
use wry::WebViewBuilder;

#[ability(webview)]
pub fn openharmony(app: OpenHarmonyApp) {
    let event_loop = EventLoopBuilder::<()>::with_user_event()
        .with_openharmony_app(app.clone())
        .build();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, _| match event {
        Event::Resumed => {
            let builder = WebViewBuilder::new()
                .with_url("http://tauri.app")
                .with_drag_drop_handler(|e| {
                    match e {
                        wry::DragDropEvent::Enter { paths, position } => {
                            println!("DragEnter: {position:?} {paths:?} ")
                        }
                        wry::DragDropEvent::Over { position } => {
                            println!("DragOver: {position:?} ")
                        }
                        wry::DragDropEvent::Drop { paths, position } => {
                            println!("DragDrop: {position:?} {paths:?} ")
                        }
                        wry::DragDropEvent::Leave => println!("DragLeave"),
                        _ => {}
                    }

                    true
                });

            let webview = builder.build(&window).unwrap();

            let w = Box::leak(Box::new(webview));
        }
        _ => {}
    });
}

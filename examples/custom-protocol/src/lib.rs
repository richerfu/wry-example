use std::cell::RefCell;

use openharmony_ability::OpenHarmonyApp;
use openharmony_ability_derive::ability;
use tao::{
    event::Event, event_loop::EventLoopBuilder, platform::ohos::EventLoopBuilderExtOpenHarmony,
    window::WindowBuilder,
};
use wry::{
    http::{header::CONTENT_TYPE, Request, Response},
    WebView, WebViewBuilder,
};

static INDEX_HTML: &str = include_str!("index.html");
static SUBPAGE_HTML: &str = include_str!("subpage.html");
static INDEX_WASM: &[u8] = include_bytes!("wasm.wasm");
static INDEX_JS: &str = include_str!("script.js");

thread_local! {
    static WEBVIEW: RefCell<Option<WebView>> = RefCell::new(None);
}

#[ability(webview, protocol = "wry")]
pub fn openharmony(app: OpenHarmonyApp) {
    let event_loop = EventLoopBuilder::<()>::with_user_event()
        .with_openharmony_app(app.clone())
        .build();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, _| match event {
        Event::Resumed => {
            // if WEBVIEW.with(|w| w.borrow().is_some()) {
            //     return;
            // }

            let builder = WebViewBuilder::new()
                .with_custom_protocol("wry".into(), move |_webview_id, request| {
                    match get_wry_response(request) {
                        Ok(r) => r.map(Into::into),
                        Err(e) => http::Response::builder()
                            .header(CONTENT_TYPE, "text/plain")
                            .status(500)
                            .body(e.to_string().as_bytes().to_vec())
                            .unwrap()
                            .map(Into::into),
                    }
                })
                // tell the webview to load the custom protocol
                .with_url("wry://localhost");

            let webview = builder.build(&window).unwrap();

            WEBVIEW.replace(Some(webview));
        }
        _ => {}
    });
}

fn get_wry_response(
    request: Request<Vec<u8>>,
) -> Result<http::Response<Vec<u8>>, Box<dyn std::error::Error>> {
    let path = request.uri().path();
    let path = if path == "/" {
        "index.html"
    } else {
        //  removing leading slash
        &path[1..]
    };
    let content = match path {
        "index.html" => INDEX_HTML.as_bytes().to_vec(),
        "subpage.html" => SUBPAGE_HTML.as_bytes().to_vec(),
        "wasm.wasm" => INDEX_WASM.to_vec(),
        "script.js" => INDEX_JS.as_bytes().to_vec(),
        _ => return Err("File not found".into()),
    };

    // Return asset contents and mime types based on file extentions
    // If you don't want to do this manually, there are some crates for you.
    // Such as `infer` and `mime_guess`.
    let mimetype = if path.ends_with(".html") || path == "/" {
        "text/html"
    } else if path.ends_with(".js") {
        "text/javascript"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".wasm") {
        "application/wasm"
    } else {
        unimplemented!();
    };

    Response::builder()
        .header(CONTENT_TYPE, mimetype)
        .body(content)
        .map_err(Into::into)
}

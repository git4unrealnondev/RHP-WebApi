#[path = "../../../src/scr/sharedtypes.rs"]
mod sharedtypes;

mod app;
mod fallback;
use crate::app::*;
//use crate::fallback::file_and_error_handler;

use async_std::task;
use std::io::Read;
use std::io::Write;

static PLUGIN_NAME: &str = "WebAPI";
static PLUGIN_DESCRIPTION: &str = "Adds support for WebUI & WebAPI..";

#[no_mangle]
pub fn return_info() -> sharedtypes::PluginInfo {
    let callbackvec = vec![sharedtypes::PluginCallback::OnStart];
    sharedtypes::PluginInfo {
        name: PLUGIN_NAME.to_string(),
        description: PLUGIN_DESCRIPTION.to_string(),
        version: 1.00,
        api_version: 1.00,
        callbacks: callbackvec,
        communication: Some(sharedtypes::PluginSharedData {
            thread: sharedtypes::PluginThreadType::Daemon,
            com_channel: Some(sharedtypes::PluginCommunicationChannel::pipe(
                "beans".to_string(),
            )),
        }),
    }
}
#[cfg(feature = "ssr")]
#[no_mangle]
pub fn on_start(reader: &mut os_pipe::PipeReader, writer: &mut os_pipe::PipeWriter) {
    task::block_on(call());
}

use futures::executor::block_on;

use std::thread;

use std::path::PathBuf;
fn get_current_working_dir() -> std::io::Result<PathBuf> {
    use std::env;
    env::current_dir()
}

/*#[actix_web::main]
async fn call() -> Server {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
}*/

use crate::app::*;
use leptos::*;
#[cfg(feature = "ssr")]
#[actix_web::main]
async fn call() -> Server {
    use axum::{routing::post, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_tailwind::{app::*, fallback::file_and_error_handler};
    use log::info;
    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, || view! { <App/> })
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    info!("listening on http://{}", &addr);
    axum::Server::bind(&addr).serve(app.into_make_service())
}

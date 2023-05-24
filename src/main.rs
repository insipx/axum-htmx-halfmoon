use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod templates;
use templates::*;
// mod serve_files;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_templates=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with some routes
    // let app = Router::new().route("/greet/:name", get(greet));

    let app = Router::new()
        .route("/", get(homepage))
        .nest_service("/static", ServeDir::new("static"));
    let app = app.fallback(get(handle_404));
    let app = add_live_reload(app);

    // run it
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// The Homepage
async fn homepage() -> HomepageTemplate<'static> {
    HomepageTemplate {
        title: "Homepage",
        header: "Yo.",
    }
}

/// Our 404 handler
async fn handle_404() -> FourOhFourTemplate<'static> {
    FourOhFourTemplate {
        title: "404: Not Found",
    }
}

#[cfg(feature = "live-reload")]
fn add_live_reload(app: Router<()>) -> Router<()> {
    app.layer(tower_livereload::LiveReloadLayer::new())
}

#[cfg(not(feature = "live-reload"))]
fn add_live_reload(app: Router<()>) -> Router<()> {
    app
}

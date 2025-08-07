use crate::{
    app_error::AppError, config::Config, highlighter::Highlighter, state::Job,
    state_task::StateTaskCtl, views::Views,
};
use anyhow::Context as _;
use anyhow::Result;
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

pub(crate) struct Web;

impl Web {
    pub(crate) async fn spawn(state_ctl: StateTaskCtl, config: &Config) -> Result<JoinHandle<()>> {
        let app = Router::new()
            .route("/jobs", get(get_jobs))
            .route("/preview", get(preview))
            .route("/jobs/output.css", get(output_css))
            .with_state((state_ctl, Highlighter::new(config)?));

        let port = config.port;
        let listener = TcpListener::bind(("127.0.0.1", port))
            .await
            .context("failed to bind")?;
        log::info!(
            "Listening on {}",
            listener.local_addr().context("failed to get local addr")?
        );

        let handle = tokio::spawn(async move {
            if let Err(err) = axum::serve(listener, app).await {
                log::error!("Failed to spawn web server: {err:?}")
            }
        });
        Ok(handle)
    }
}

async fn get_jobs(
    State((state, highlighter)): State<(StateTaskCtl, Highlighter)>,
) -> Result<Html<String>, AppError> {
    let (post, jobs) = state.get().await?;

    const HIGHLIGHT_PRE: &str = r#"<span class="highlight-container"><span class="highlight">"#;
    const HIGHLIGHT_POST: &str = r#"</span></span>"#;

    let jobs = jobs
        .into_iter()
        .map(|mut job| {
            highlighter.highlight(&mut job.text, HIGHLIGHT_PRE, HIGHLIGHT_POST);
            job
        })
        .collect::<Vec<_>>();

    let html = Views::index(post, jobs)?;
    Ok(Html(html))
}

async fn preview() -> Result<Html<String>, AppError> {
    let html = Views::jobs_email(vec![Job::fixture(); 10])?;
    Ok(Html(html))
}

async fn output_css() -> impl IntoResponse {
    let css = Views::output_css();
    (StatusCode::OK, [("content-type", "text/css")], css)
}

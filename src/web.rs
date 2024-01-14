use anyhow::Context;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tokio::net::TcpListener;

use crate::{
    app_error::AppError, config::Config, fixture::Fixture, job::Job, post::Post, state::AppState,
    views::Views,
};

pub(crate) struct Web;

impl Web {
    pub(crate) async fn spawn(state: AppState) -> anyhow::Result<()> {
        let app = Router::new()
            .route("/jobs", get(get_jobs))
            .route("/preview", get(preview))
            .route("/jobs/output.css", get(output_css))
            .with_state(state);

        let port = Config::global()?.listen_on;
        let listener = TcpListener::bind(("0.0.0.0", port))
            .await
            .context("failed to bind")?;
        println!(
            "Listening on {}",
            listener.local_addr().context("failed to get local addr")?
        );

        axum::serve(listener, app)
            .await
            .context("Failed to spawn web server")
    }
}

async fn get_jobs(
    State(AppState { database, .. }): State<AppState>,
) -> Result<Html<String>, AppError> {
    let post = database.last_post().await?.unwrap_or_else(Post::fixture);

    let jobs: Vec<Job> = database
        .list_jobs(post.hn_id)
        .await?
        .unwrap_or_else(|| vec![Job::fixture(); 10])
        .into_iter()
        .map(|job| job.highlight_keywords(highlight_one_keyword))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let html = Views::index(&post, &jobs)?;
    Ok(Html(html))
}

async fn preview() -> Result<Html<String>, AppError> {
    let html = Views::jobs_email(&vec![Job::fixture(); 10])?;
    Ok(Html(html))
}

async fn output_css() -> impl IntoResponse {
    let css = Views::output_css();
    (StatusCode::OK, [("content-type", "text/css")], css)
}

fn highlight_one_keyword(keyword: &str) -> String {
    format!(
        r#"
            <span class="highlight-container">
                <span class="highlight">
                    {}
                </span>
            </span>
        "#,
        keyword
    )
}

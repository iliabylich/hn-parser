use axum::{extract::State, response::Html, routing::get, Router, Server};
use std::net::SocketAddr;

use crate::{fixture::Fixture, job::Job, post::Post, state::AppState};

pub(crate) struct Web;

impl Web {
    pub(crate) async fn spawn(state: AppState) {
        let app = Router::new()
            .route("/jobs", get(Self::get_jobs))
            .route("/preview", get(Self::preview))
            .with_state(state);

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        println!("Listening on {}", addr);

        Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }

    async fn get_jobs(State(state): State<AppState>) -> Html<String> {
        let db = &state.database;
        let post = db.last_post().await.unwrap_or_else(Post::fixture);
        let mut jobs = db.list_jobs(post.hn_id).await;
        if jobs.is_empty() {
            jobs = vec![Job::fixture(); 10];
        }
        for job in &mut jobs {
            job.highlight_keywords(Self::highlight_one_keyword);
        }
        let html = state.views.index(&post, &jobs);
        Html(html)
    }

    async fn preview(State(state): State<AppState>) -> Html<String> {
        let jobs = vec![Job::fixture(); 10];
        let html = state.views.jobs_email(&jobs);
        Html(html)
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
}

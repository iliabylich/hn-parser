use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
    routing::{get, post},
    Router, Server,
};
use std::net::SocketAddr;

use crate::{job::Job, state::AppState};

pub(crate) struct Web;

impl Web {
    pub(crate) async fn spawn(state: AppState) {
        let app = Router::new()
            .route("/jobs", get(Self::get_jobs))
            .route("/jobs/:id", post(Self::mark_job_as_read))
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
        let post = db.last_post().await.unwrap_or_default();
        let mut jobs = db.list_jobs(post.hn_id).await;
        if jobs.is_empty() {
            jobs = vec![Job::default(); 10];
        }
        for job in &mut jobs {
            job.highlight_keywords(Self::highlight_one_keyword);
        }
        jobs.push(Job::default());
        let html = state.views.index(&post, &jobs);
        Html(html)
    }

    async fn mark_job_as_read(
        State(_state): State<AppState>,
        Path(post_id): Path<u64>,
    ) -> Redirect {
        println!("marking job as read {}", post_id);
        Redirect::to("/jobs")
    }

    fn highlight_one_keyword(keyword: &str) -> String {
        format!(
            "<span class=\"highlight-container\"><span class=\"highlight\">{}</span></span>",
            keyword
        )
    }
}

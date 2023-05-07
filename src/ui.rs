use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
    routing::{get, post},
    Router, Server,
};
use std::net::SocketAddr;

use crate::state::AppState;

pub(crate) struct UI;

impl UI {
    pub(crate) async fn spawn(state: AppState) {
        let app = Router::new()
            .route("/jobs", get(UI::get_jobs))
            .route("/jobs/:id", post(UI::mark_job_as_read))
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
        let post = db.last_post().await.unwrap();
        let jobs = db.list_jobs(post.hn_id).await;
        Html(format!("{:?}\n{:?}", post, jobs))
    }

    async fn mark_job_as_read(
        State(_state): State<AppState>,
        Path(post_id): Path<u64>,
    ) -> Redirect {
        println!("marking job as read {}", post_id);
        Redirect::to("/jobs")
    }
}

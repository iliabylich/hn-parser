use axum::{
    routing::{get, post},
    Router, Server,
};
use std::net::SocketAddr;

pub(crate) struct UI;

impl UI {
    pub(crate) async fn spawn() {
        let app = Router::new()
            .route("/jobs", get(UI::get_jobs))
            .route("/users", post(UI::mark_job_as_read));

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        println!("Listening on {}", addr);

        Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }

    async fn get_jobs() -> &'static str {
        "Jobs"
    }

    async fn mark_job_as_read() -> &'static str {
        "Mark job as read"
    }
}

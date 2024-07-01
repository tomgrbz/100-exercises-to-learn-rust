// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

pub mod data;
pub mod store;

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post, Router},
    Error, Json,
};
use axum_macros::debug_handler;
use data::{Ticket, TicketDraft};
use store::{TicketId, TicketStore};
use ticket_fields::{TicketDescription, TicketTitle};
use tokio::sync::Mutex;

pub async fn launch() -> Result<(), Error> {
    let ticket_store = Arc::new(Mutex::new(TicketStore::new()));
    ticket_store.lock().await.add_ticket(TicketDraft {
        title: TicketTitle::try_from("Test Title").unwrap(),
        description: TicketDescription::try_from("Desc").unwrap(),
    });
    let app = Router::new()
        .route("/tickets", post(create_ticket))
        .route("/tickets/:id", get(fetch_ticket))
        .route("/tickets/update/:id", patch(patch_ticket))
        .with_state(ticket_store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[debug_handler]
async fn create_ticket(
    State(ticket_store): State<Arc<Mutex<TicketStore>>>,
    Json(payload): Json<TicketDraft>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = ticket_store.lock().await.add_ticket(payload);
    return Ok(Json(id));
}

async fn fetch_ticket(
    Path(ticket_id): Path<TicketId>,
    State(ticket_store): State<Arc<Mutex<TicketStore>>>,
) -> Result<impl IntoResponse, StatusCode> {
    let tix = ticket_store.lock().await.get(ticket_id);
    if let Some(t) = tix {
        match t.read() {
            Ok(v) => Ok(Json(v.clone())),
            Err(e) => Err(StatusCode::BAD_REQUEST),
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

async fn patch_ticket(
    Path(ticket_id): Path<TicketId>,
    State(ticket_store): State<Arc<Mutex<TicketStore>>>,
    Json(ticket_details): Json<Ticket>
) -> Result<impl IntoResponse, StatusCode> {
    let tix = ticket_store.lock().await.get(ticket_id);
    if let Some(t) = tix {
        match t.write() {
            Ok(mut v) => {
                v.title = ticket_details.title;
                v.description = ticket_details.description;
                Ok(Json(v.clone()))
            },
            Err(e) => Err(StatusCode::BAD_REQUEST),
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

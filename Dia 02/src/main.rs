pub mod db;

// from axum.extract import Path, State
// from axum.routing import get, post
// from axum import Json, Router
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};

use db::{Db, Item};
use serde_json::{json, Value};

async fn get_item(State(db): State<Db>, Path(name): Path<String>) -> Json<Option<Item>> {
    let item = db.get(&name);
    Json(item)
}

async fn insert_item(State(db): State<Db>, Json(item): Json<Item>) -> Json<Value> {
    db.insert(item);
    Json(json!({ "success": true }))
}

async fn delete_item(State(db): State<Db>, Path(name): Path<String>) -> Json<Value> {
    db.delete(&name);
    Json(json!({ "success": true }))
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", post(insert_item))
        .route("/:name", get(get_item).delete(delete_item))
        .with_state(Db::new());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
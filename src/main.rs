use std::{collections::HashMap, sync::Mutex};

use axum::{
    response::Html, 
    response::Json, 
    routing::get, 
    routing::post, 
    Router};

use serde_json::{Value, json};
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize)]
#[derive(Debug)]
// Struct che serde deserializza dal JSON ricevuto in POST
struct Metric {
    name: String,
    value: i64,
    tags: HashMap<String, String>
}
//variabile globale utilizzata per salvare in memoria le metriche
static METRICS: Mutex<Vec<Metric>> = Mutex::new(Vec::new());

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/health-check", get(health_check))
        .route("/metric", post(metric_handler));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn health_check() -> Json<Value> {
    Json(json!({ "OK": 200 }))
}
/* Json<Metric> = "Mi aspetto JSON che rappresenta una Metric"
Serde deserializza = Converte il JSON ricevuto in una struct Metric (perché ha #[derive(Deserialize)])
Json(v) = Pattern matching che estrae la Metric dal wrapper e la chiama v*/
async fn metric_handler(Json(v): Json<Metric>) -> Json<Value> {
    /*  lock() acquisisce il mutex (sincronizza l'accesso se più thread lo usano)
        unwrap blocca il codice se l'oggetto chiamante è un errore
        push, pusha la metrica in METRICS
    */
    METRICS.lock().unwrap().push(v);
    log_metrics(&METRICS);
    Json(json!({ "OK": 200 }))
}

fn log_metrics<T: Debug>(metrics: &Mutex<Vec<T>>) {
    println!("Metrica ricevuta: {:?}", metrics.lock().unwrap());
}
use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions}; 
use std::str::FromStr;
use sqlx::{SqlitePool, Row};

// Define our table structs 
#[derive(Clone)]
struct AppState { // Holds the server state information SqlitePool (DB connection information, Redis information)
    db: SqlitePool,
}

// Table Struct 
// struct Redirect {
//     id: i64,
//     url: String,
//     active: bool, // If we want to deactivate it in the future
// }

//Post Request STruct
#[derive(Deserialize)]
struct RedirectPost {
    url: String,
}

struct RedirectResponse {
    id: i64,
    url: String,
    active: bool, // If we want to deactivate it in the future
}

#[tokio::main]
async fn main() {

    // Define SQLite creds
    let connection_options = SqliteConnectOptions::from_str("sqlite:redirects.db")
        .unwrap()
        .create_if_missing(true);

    // opens db connection
    let pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await
        .expect("Failed to connect or create SQLite database");


    // Create your redirects table if it doesn't exist yet
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS redirects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            url TEXT NOT NULL,
            active BOOLEAN NOT NULL DEFAULT 1
        );"
    )
    .execute(&pool)
    .await
    .expect("Failed to initialize tables");

    println!("📊 Success: Database tables verified and ready!");

    let state = AppState { db: pool };

    // build our application with a single route
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/redirects", post(createRedirectUrl))
    .route("/{id}", get(getRedirectUrl))
    .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}


// new function 
// 1. Parse JSON from user in POST req
// 2. Validate JSON whether URL is valid string
// 3. Write to DB and get it's ID in the redirects table 
// 4. Return to the user a response with id or redirect URL link 

async fn createRedirectUrl(
    State(state): State<AppState>,
    Json(payload): Json<RedirectPost>,
) -> impl IntoResponse {
    // How do we create a POST request? 
    // How do we redirect? 
    let result = sqlx::query("INSERT INTO redirects (url) VALUES (?)")
        .bind(payload.url) // We read from the variable we just invented
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn getRedirectUrl(
    State(state): State<AppState>,
    Path(redirect_id): Path<i64>,
) -> impl IntoResponse {
    // Check if ID exists? 
    // If ID exists match result, Ok, then we redirect
    // If it doesn't exist, we throw an internal server error. 
    let row = sqlx::query("SELECT id, url, active FROM redirects WHERE id = ?")
    .bind(redirect_id)
    .fetch_optional(&state.db)
    .await;

    match row {
        Ok(Some(row)) => {
            let response = RedirectResponse {
                id: row.get("id"),
                url: row.get("url"),
                active: row.get("active")
            };

            if response.active {
                Redirect::to(&response.url).into_response()
            } else {
                StatusCode::NOT_FOUND.into_response()
            }
        }
        Ok(None) => {
            StatusCode::NOT_FOUND.into_response()
        }
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }

    // let row = sqlx::query("SELECT id, url, active FROM redirects WHERE id = ?")
    //     .bind(redirect_id) 
    //     .fetch_optional(&state.db)
    //     .await;
}


















// 1. SQLite (Done)
// 2. /{id} -> redirect to table id 1 = url 
// 3. axum 

// 1. We have our server running 
// 2. Database 

// Functional Requirements 
// 1. Redirect URLs 
// 2. Create URL shorteners accept an input body post req 
// {
//     "url" : String::from()
// }
// 3. Save in the DB with this post req parse the json from post req, check whether URL is valid string, if valid, save into url table, if invalid, throw an error 
// 4. Redirect part, get req, /{id} 
// 5. If id exists and has url, we redirect to the URL saved in table, if id  doesn't exist, we return a http error. 

// How do we actually run an SQLite instance with Rust here

// what is connection_options

// what is pool 

    // // Configure SQLite to automatically create "redirects.db" if it doesn't exist
    // let connection_options = SqliteConnectOptions::from_str("sqlite:redirects.db")
    //     .unwrap()
    //     .create_if_missing(true);

    // // Create the connection pool
    // let pool = SqlitePoolOptions::new()
    //     .connect_with(connection_options)
    //     .await
    //     .expect("Failed to connect or create SQLite database");

    // // Create your redirects table if it doesn't exist yet
    // sqlx::query(
    //     "CREATE TABLE IF NOT EXISTS redirects (
    //         id INTEGER PRIMARY KEY AUTOINCREMENT,
    //         url TEXT NOT NULL,
    //         active BOOLEAN NOT NULL DEFAULT 1
    //     );"
    // )
    // .execute(&pool)
    // .await
    // .expect("Failed to initialize tables");

    // // Initialize your AppState container with the database pool
    // let state = AppState { db: pool };
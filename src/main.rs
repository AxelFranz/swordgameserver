use std::env;

use rocket::{http::Status, response::Redirect};
use dotenv::dotenv;
use sqlx::PgPool;
use rocket::serde::{Serialize, Deserialize, json::Json};


#[macro_use] extern crate rocket;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

struct Score {
    key: i64,
    username: String,
    score: i64
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Stats {
    scores: Vec<Score>
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/list")
}


#[get("/list")]
async fn list_scores(pool: &rocket::State<PgPool>) -> Json<Stats> {
    let scores: Vec<Score> = sqlx::query_as("SELECT * FROM SCORES").fetch_all(pool.inner()).await.unwrap();
    Json(
        Stats{
            scores:scores
        }
    )
}

#[post("/new?<score>&<username>")]
async fn add_score(pool: &rocket::State<PgPool>,score: u32, username: String) -> Status {
    println!("{} {}",score, username);
    Status::Created
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(database_url.as_str())
        .await
        .expect("Failed to connect to database")
    ;
    rocket::build()
    .manage::<PgPool>(pool)
    .mount("/", routes![index, list_scores, add_score])
}

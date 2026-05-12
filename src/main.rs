use rocket::{http::Status, response::Redirect};

#[macro_use] extern crate rocket;

struct Score {
    username: String,
    score: u32
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/list")
}


#[get("/list")]
fn list_scores() -> String {
    "coucou".to_string()
}

#[get("/new?<score>&<username>")]
fn add_score(score: u32, username: String) -> Status {
    println!("{} {}",score, username);
    Status::Created
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, list_scores, add_score])
}

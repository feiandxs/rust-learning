#[macro_use] extern crate rocket;

mod schema;
use schema::{Response1, Response2};

#[get("/response1")]
fn handler1() -> rocket::serde::json::Json<Response1> {
    // let data: Vec<i32> = (1..=100).map(|x| x).collect::<Vec<_>>();
    let data: Vec<i32> = (1..=100).collect();
    let response = Response1 {
        message: "This is response1".to_string(),
        data: data,
    };
    rocket::serde::json::Json(response)
}

#[get("/response2")]
fn handler2() -> rocket::serde::json::Json<Response2> {
    let response = Response2 {
        status: true,
        result: "This is response2".to_string(),
    };
    rocket::serde::json::Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![handler1, handler2])
}
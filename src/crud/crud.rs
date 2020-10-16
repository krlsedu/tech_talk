use super::super::structs::structs::UserBestIde;
use super::super::structs::structs::Message;

use rocket_contrib::json::{Json, JsonValue};

#[post("/", data = "<user_best_ide>")]
fn post(user_best_ide: Json<UserBestIde>) -> Json<JsonValue> {
    let message = Message { message: user_best_ide.show_result() };
    Json(json!(message))
}

pub fn start() {
    rocket::ignite()
        .mount("/hello", routes![post])
        .launch();
}
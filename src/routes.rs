use rocket_contrib::json::JsonValue;

#[get("/")]
pub fn index() -> Result<JsonValue, &'static str> {
    Ok(json!({"ok": true}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    #[test]
    fn test_get_index() {
        let rocket = rocket::ignite().mount("/", routes![index]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let req = client.get("/");
        let mut response = req.dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        assert_eq!(response.body_string(), Some("{\"ok\":true}".into()));
    }
}

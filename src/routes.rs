use crate::beer;
use rocket_contrib::json::{Json, JsonValue};

#[get("/")]
pub fn index() -> Result<JsonValue, &'static str> {
    Ok(json!({"ok": true}))
}

#[post("/malt", data = "<recipe>")]
pub fn post_malt(recipe: Json<beer::MaltRequest>) -> Result<JsonValue, &'static str> {
    Ok(json!(recipe.to_response()))
}

#[post("/hop", data = "<recipe>")]
pub fn post_hop(recipe: Json<beer::HopRequest>) -> Result<JsonValue, &'static str> {
    Ok(json!(recipe.to_response()))
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
        let mut resp = client.get("/").dispatch();

        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(resp.body_string(), Some("{\"ok\":true}".into()));
    }

    #[test]
    fn test_post_malt() {
        let rocket = rocket::ignite().mount("/", routes![post_malt]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut resp = client
            .post("/malt")
            .body(
                "{
                \"efficiency\":80,
                \"quantity\":20,
                \"malts\":[{\"quantity\":1500,\"ebc\":10}]
            }",
            )
            .dispatch();

        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(
            resp.body_string(),
            Some("{\"color\":[248,166,0],\"ebc\":6.0}".into())
        );
    }

    #[test]
    fn test_post_malt_empty_body() {
        let rocket = rocket::ignite().mount("/", routes![post_malt]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut resp = client.post("/malt").body("{}").dispatch();

        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(
            resp.body_string(),
            Some("{\"color\":[0,0,0],\"ebc\":null}".into())
        );
    }

    #[test]
    fn test_post_hop() {
        let rocket = rocket::ignite().mount("/", routes![post_hop]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut resp = client
            .post("/hop")
            .body(
                "{
                \"quantity\":20,
                \"original_gravity\":1020,
                \"hops\":[{\"quantity\":12,\"alpha\":12,\"duration\":10}]
            }",
            )
            .dispatch();

        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(
            resp.body_string(),
            Some("{\"ibu\":7.880000114440918}".into())
        );
    }

    #[test]
    fn test_post_hop_empty_body() {
        let rocket = rocket::ignite().mount("/", routes![post_hop]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut resp = client.post("/hop").body("{}").dispatch();

        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(resp.body_string(), Some("{\"ibu\":0.0}".into()));
    }
}

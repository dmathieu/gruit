use crate::beer;
use rocket_contrib::json::{Json, JsonValue};

#[get("/")]
pub fn index() -> Result<JsonValue, &'static str> {
    Ok(json!({"ok": true}))
}

#[post("/beer", data = "<recipe>")]
pub fn post_beer(recipe: Json<beer::RecipeRequest>) -> Result<JsonValue, &'static str> {
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
    fn test_post_beer() {
        let rocket = rocket::ignite().mount("/", routes![post_beer]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut resp = client
            .post("/beer")
            .body(
                "{
                \"efficiency\":80,
                \"quantity\":20,
                \"original_gravity\":1020,
                \"malts\":[{\"quantity\":1500,\"ebc\":10}],
                \"hops\":[{\"quantity\":12,\"alpha\":12,\"duration\":10}]
            }",
            )
            .dispatch();

        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(
            resp.body_string(),
            Some("{\"color\":[248,166,0],\"ebc\":6,\"ibu\":7.880000114440918}".into())
        );
    }

    #[test]
    fn test_post_beer_empty_body() {
        let rocket = rocket::ignite().mount("/", routes![post_beer]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut resp = client.post("/beer").body("{}").dispatch();

        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(
            resp.body_string(),
            Some("{\"color\":[0,0,0],\"ebc\":0,\"ibu\":0.0}".into())
        );
    }
}

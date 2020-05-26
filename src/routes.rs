use crate::beer;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

#[get("/")]
pub fn index() -> Result<JsonValue, &'static str> {
    Ok(json!({"ok": true}))
}

#[derive(Deserialize)]
pub struct MaltRequest {
    name: String,
    ebc: i32,
}

#[derive(Deserialize)]
pub struct RecipeRequest {
    malts: Vec<MaltRequest>,
}

#[post("/beer", data = "<recipe_request>")]
pub fn post_beer(recipe_request: Json<RecipeRequest>) -> Result<JsonValue, &'static str> {
    let mut recipe = beer::Recipe { malts: Vec::new() };
    for malt in &recipe_request.malts {
        recipe.malts.push(beer::Malt {
            name: malt.name.clone(),
            ebc: malt.ebc,
        });
    }

    Ok(json!(recipe))
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
            .body("{\"malts\":[{\"ebc\":10,\"name\":\"hello\"}]}")
            .dispatch();

        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(
            resp.body_string(),
            Some("{\"malts\":[{\"ebc\":10,\"name\":\"hello\"}]}".into())
        );
    }
}

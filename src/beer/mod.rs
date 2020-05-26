use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RecipeRequest {
    malts: Vec<Malt>,
}

impl RecipeRequest {
    pub fn to_response(&self) -> RecipeResponse {
        let mut recipe = RecipeResponse { malts: Vec::new() };
        for malt in &self.malts {
            recipe.malts.push(Malt {
                name: malt.name.clone(),
                ebc: malt.ebc,
            });
        }

        recipe
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Malt {
    pub name: String,
    pub ebc: i32,
}

#[derive(Serialize, Debug)]
pub struct RecipeResponse {
    pub malts: Vec<Malt>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_request() {
        let recipe = RecipeRequest {
            malts: vec![Malt {
                name: String::from("hello"),
                ebc: 10,
            }],
        };
        assert_eq!(1, recipe.malts.len());

        let response = recipe.to_response();
        assert_eq!(1, response.malts.len());
    }

    #[test]
    fn test_recipe_deserialization() {
        let deserialized: RecipeRequest =
            serde_json::from_str("{\"malts\":[{\"name\":\"carared\",\"ebc\":10}]}").unwrap();
        assert_eq!(deserialized.malts[0].name, "carared");
    }

    #[test]
    fn test_recipe_serialization() {
        let recipe = RecipeResponse {
            malts: vec![Malt {
                name: String::from("carabelge"),
                ebc: 10,
            }],
        };

        let serialized = serde_json::to_string(&recipe).unwrap();
        assert_eq!(
            "{\"malts\":[{\"name\":\"carabelge\",\"ebc\":10}]}",
            serialized
        );
    }
}

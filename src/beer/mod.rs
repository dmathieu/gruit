use serde::{Deserialize, Serialize};
mod malt;

#[derive(Deserialize)]
pub struct RecipeRequest {
    efficiency: Option<i32>,
    quantity: Option<i32>,
    malts: Option<Vec<malt::Malt>>,
}

impl RecipeRequest {
    pub fn to_response(&self) -> RecipeResponse {
        let empty_malt: Vec<malt::Malt> = Vec::new();
        let malts = self.malts.as_ref().unwrap_or(&empty_malt);

        RecipeResponse {
            ebc: malt::calculate_ebc(
                self.efficiency.unwrap_or(0),
                self.quantity.unwrap_or(0),
                &malts,
            ),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct RecipeResponse {
    ebc: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_request() {
        let recipe = RecipeRequest {
            efficiency: Some(80),
            quantity: Some(20),
            malts: Some(vec![malt::Malt {
                ebc: 10,
                quantity: 500,
            }]),
        };

        let malts = recipe.malts.as_ref().unwrap();
        assert_eq!(1, malts.len());

        let response = recipe.to_response();
        assert_eq!(2, response.ebc);
    }

    #[test]
    fn test_recipe_deserialization() {
        let deserialized: RecipeRequest = serde_json::from_str(
            "{\"efficiency\":75,\"quantity\":50,\"malts\":[{\"ebc\":10,\"quantity\":200}]}",
        )
        .unwrap();
        assert_eq!(deserialized.malts.unwrap()[0].ebc, 10);
    }

    #[test]
    fn test_recipe_serialization() {
        let recipe = RecipeResponse { ebc: 10 };

        let serialized = serde_json::to_string(&recipe).unwrap();
        assert_eq!("{\"ebc\":10}", serialized);
    }
}

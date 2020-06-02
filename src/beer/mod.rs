use serde::{Deserialize, Serialize};
mod malt;

#[derive(Deserialize)]
pub struct RecipeRequest {
    #[serde(default)]
    efficiency: i32,
    #[serde(default)]
    quantity: i32,
    #[serde(default)]
    malts: Vec<malt::Malt>,
}

impl RecipeRequest {
    pub fn to_response(&self) -> RecipeResponse {
        let ebc = malt::calculate_ebc(self.efficiency, self.quantity, &self.malts);

        RecipeResponse {
            ebc: ebc,
            color: malt::calculate_color(ebc),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct RecipeResponse {
    ebc: i32,
    color: malt::Color,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_request() {
        let recipe = RecipeRequest {
            efficiency: 80,
            quantity: 20,
            malts: vec![malt::Malt {
                ebc: 10,
                quantity: 500,
            }],
        };

        assert_eq!(1, recipe.malts.len());

        let response = recipe.to_response();
        assert_eq!(2, response.ebc);
    }

    #[test]
    fn test_recipe_deserialization() {
        let deserialized: RecipeRequest = serde_json::from_str(
            "{\"efficiency\":75,\"quantity\":50,\"malts\":[{\"ebc\":10,\"quantity\":200}]}",
        )
        .unwrap();
        assert_eq!(deserialized.malts[0].ebc, 10);
    }

    #[test]
    fn test_recipe_serialization() {
        let recipe = RecipeResponse {
            ebc: 10,
            color: malt::calculate_color(10),
        };

        let serialized = serde_json::to_string(&recipe).unwrap();
        assert_eq!("{\"ebc\":10,\"color\":[222,124,0]}", serialized);
    }
}

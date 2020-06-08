use serde::{Deserialize, Serialize};
mod hops;
mod malt;

#[derive(Deserialize)]
pub struct RecipeRequest {
    #[serde(default)]
    efficiency: i32,
    #[serde(default)]
    quantity: i32,
    #[serde(default)]
    original_gravity: i32,
    #[serde(default)]
    malts: Vec<malt::Malt>,
    #[serde(default)]
    hops: Vec<hops::Hop>,
}

impl RecipeRequest {
    pub fn to_response(&self) -> RecipeResponse {
        let ebc = malt::calculate_ebc(self.efficiency, self.quantity, &self.malts);

        RecipeResponse {
            ebc: ebc,
            color: malt::calculate_color(ebc),
            ibu: hops::calculate_ibu(self.quantity, self.original_gravity, &self.hops),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct RecipeResponse {
    ebc: i32,
    color: malt::Color,
    ibu: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_request() {
        let recipe = RecipeRequest {
            efficiency: 80,
            quantity: 20,
            original_gravity: 1020,
            malts: vec![malt::Malt {
                ebc: 10,
                quantity: 500,
            }],
            hops: vec![hops::Hop {
                quantity: 10,
                alpha: 6,
                duration: 30,
            }],
        };

        assert_eq!(1, recipe.malts.len());
        assert_eq!(1, recipe.hops.len());

        let response = recipe.to_response();
        assert_eq!(2, response.ebc);
    }

    #[test]
    fn test_recipe_deserialization() {
        let deserialized: RecipeRequest = serde_json::from_str(
            "{
                \"efficiency\":75,
                \"quantity\":50,
                \"original_gravity\":1030,
                \"malts\":[{\"ebc\":10,\"quantity\":200}],
                \"hops\":[{\"quantity\":7,\"alpha\":12,\"duration\":60}]
            }",
        )
        .unwrap();
        assert_eq!(deserialized.malts[0].ebc, 10);
    }

    #[test]
    fn test_recipe_serialization() {
        let recipe = RecipeResponse {
            ebc: 10,
            color: malt::calculate_color(10),
            ibu: 40.0,
        };

        let serialized = serde_json::to_string(&recipe).unwrap();
        assert_eq!(
            "{\"ebc\":10,\"color\":[222,124,0],\"ibu\":40.0}",
            serialized
        );
    }
}

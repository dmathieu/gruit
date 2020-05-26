use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Malt {
    pub name: String,
    pub ebc: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub malts: Vec<Malt>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_recipe() {
        let recipe = Recipe { malts: Vec::new() };
        assert_eq!(0, recipe.malts.len());
    }

    #[test]
    fn test_recipe_serialization() {
        let recipe = Recipe {
            malts: vec![Malt {
                name: String::from("hello"),
                ebc: 10,
            }],
        };

        let serialized = serde_json::to_string(&recipe).unwrap();
        let deserialized: Recipe = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.malts[0].name, recipe.malts[0].name);
    }
}

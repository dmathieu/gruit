use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Malt {
    pub name: String,
    pub quantity: i32,
    pub ebc: i32,
}

pub fn calculate_ebc(efficiency: i32, quantity: i32, malts: &Vec<Malt>) -> i32 {
    let mut sum = 0.0;
    for malt in malts {
        sum = sum + ((malt.quantity as f32 / 1000 as f32) * malt.ebc as f32);
    }
    ((sum * 10.0 * (efficiency as f32 / 100.0)) / quantity as f32) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_ebc() {
        assert_eq!(0, calculate_ebc(80, 20, &Vec::new()));
        assert_eq!(
            60,
            calculate_ebc(
                80,
                20,
                &vec![Malt {
                    name: String::from("test name"),
                    ebc: 150,
                    quantity: 1000
                }]
            )
        );
        assert_eq!(
            64,
            calculate_ebc(
                80,
                20,
                &vec![
                    Malt {
                        name: String::from("test name"),
                        ebc: 150,
                        quantity: 1000
                    },
                    Malt {
                        name: String::from("test name"),
                        ebc: 20,
                        quantity: 500
                    },
                ]
            )
        );
    }
}

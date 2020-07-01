use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Hop {
    pub quantity: i32,
    pub alpha: f32,
    pub duration: i32,
}

// Calculating coming from http://www.backtoschoolbrewing.com/blog/2016/9/5/how-to-calculate-ibus
pub fn calculate_ibu(quantity: i32, gravity: i32, hops: &Vec<Hop>) -> f32 {
    let mut sum = 0 as f64;
    for hop in hops {
        sum = sum
            + (hop.quantity as f64
                * utilization(hop.duration, gravity)
                * hop.alpha as f64
                * 1000.0)
                / quantity as f64;
    }

    sum.trunc() as f32 / 100.0
}

// Calculation coming from http://howtobrew.com/book/section-1/hops/hop-bittering-calculations
pub fn utilization(duration: i32, gravity: i32) -> f64 {
    let bigness = 1.65 * (0.000125 as f64).powf((gravity as f64 / 1000.0) - 1.0);
    let time_factor = (1.0 - std::f64::consts::E.powf(-0.04 * duration as f64)) / 4.15;
    bigness * time_factor
}

#[test]
fn test_calculate_ibu() {
    assert_eq!(0.0, calculate_ibu(10, 1050, &Vec::new()));
    assert_eq!(
        10.63,
        calculate_ibu(
            10,
            1050,
            &vec![Hop {
                quantity: 10,
                alpha: 6.0,
                duration: 30
            }]
        )
    );
    assert_eq!(
        21.92,
        calculate_ibu(
            20,
            1050,
            &vec![
                Hop {
                    quantity: 10,
                    alpha: 6.0,
                    duration: 30
                },
                Hop {
                    quantity: 12,
                    alpha: 12.0,
                    duration: 60
                },
            ]
        )
    );
}

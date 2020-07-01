use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Malt {
    pub quantity: i32,
    pub ebc: f32,
}

#[derive(Serialize, Debug)]
pub struct Color(i32, i32, i32);

pub fn calculate_ebc(efficiency: i32, quantity: i32, malts: &Vec<Malt>) -> f32 {
    let mut sum = 0.0;
    for malt in malts {
        sum = sum + ((malt.quantity as f32 / 1000 as f32) * malt.ebc);
    }
    (sum * 10.0 * (efficiency as f32 / 100.0)) / quantity as f32
}

pub fn calculate_color(ebc: f32) -> Color {
    match ebc as i32 {
        1 => Color(255, 230, 153),
        2 => Color(255, 216, 120),
        3 => Color(255, 202, 90),
        4 => Color(255, 191, 66),
        5 => Color(251, 177, 35),
        6 => Color(248, 166, 0),
        7 => Color(243, 156, 0),
        8 => Color(234, 143, 0),
        9 => Color(229, 133, 0),
        10 => Color(222, 124, 0),
        11 => Color(215, 114, 0),
        12 => Color(207, 105, 0),
        13 => Color(203, 98, 0),
        14 => Color(195, 89, 0),
        15 => Color(187, 81, 0),
        16 => Color(181, 76, 0),
        17 => Color(176, 69, 0),
        18 => Color(166, 62, 0),
        19 => Color(161, 55, 0),
        20 => Color(155, 50, 0),
        21 => Color(149, 45, 0),
        22 => Color(142, 41, 0),
        23 => Color(136, 35, 0),
        24 => Color(130, 30, 0),
        25 => Color(123, 26, 0),
        26 => Color(119, 25, 0),
        27 => Color(112, 20, 0),
        28 => Color(106, 14, 0),
        29 => Color(102, 13, 0),
        30 => Color(94, 11, 0),
        31 => Color(90, 10, 2),
        32 => Color(96, 9, 3),
        33 => Color(82, 9, 7),
        34 => Color(76, 5, 5),
        35 => Color(71, 6, 6),
        36 => Color(68, 6, 7),
        37 => Color(63, 7, 8),
        38 => Color(59, 6, 7),
        39 => Color(58, 7, 11),
        40 => Color(54, 8, 10),
        _ => Color(0, 0, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_ebc() {
        assert_eq!(0.0, calculate_ebc(80, 20, &Vec::new()));
        assert_eq!(
            60.0,
            calculate_ebc(
                80,
                20,
                &vec![Malt {
                    ebc: 150.0,
                    quantity: 1000
                }]
            )
        );
        assert_eq!(
            64.0,
            calculate_ebc(
                80,
                20,
                &vec![
                    Malt {
                        ebc: 150.0,
                        quantity: 1000
                    },
                    Malt {
                        ebc: 20.0,
                        quantity: 500
                    },
                ]
            )
        );
    }

    #[test]
    fn test_calculate_color() {
        let mut color = calculate_color(1.0);
        assert_eq!(255, color.0);
        assert_eq!(230, color.1);
        assert_eq!(153, color.2);

        color = calculate_color(25.0);
        assert_eq!(123, color.0);
        assert_eq!(26, color.1);
        assert_eq!(0, color.2);

        color = calculate_color(70.0);
        assert_eq!(0, color.0);
        assert_eq!(0, color.1);
        assert_eq!(0, color.2);
    }
}

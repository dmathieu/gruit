use serde::{Deserialize, Serialize};
mod hops;
mod malt;

#[derive(Deserialize)]
pub struct MaltRequest {
    #[serde(default)]
    efficiency: i32,
    #[serde(default)]
    quantity: i32,
    #[serde(default)]
    malts: Vec<malt::Malt>,
}

impl MaltRequest {
    pub fn to_response(&self) -> MaltResponse {
        let ebc = malt::calculate_ebc(self.efficiency, self.quantity, &self.malts);

        MaltResponse {
            ebc: ebc,
            color: malt::calculate_color(ebc),
        }
    }
}

#[derive(Deserialize)]
pub struct HopRequest {
    #[serde(default)]
    quantity: i32,
    #[serde(default)]
    original_gravity: i32,
    #[serde(default)]
    hops: Vec<hops::Hop>,
}

impl HopRequest {
    pub fn to_response(&self) -> HopResponse {
        HopResponse {
            ibu: hops::calculate_ibu(self.quantity, self.original_gravity, &self.hops),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MaltResponse {
    ebc: f32,
    color: malt::Color,
}

#[derive(Serialize, Debug)]
pub struct HopResponse {
    ibu: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_malt_request() {
        let req = MaltRequest {
            efficiency: 80,
            quantity: 20,
            malts: vec![malt::Malt {
                ebc: 10.0,
                quantity: 500,
            }],
        };

        assert_eq!(1, req.malts.len());

        let resp = req.to_response();
        assert_eq!(2.0, resp.ebc);
    }

    #[test]
    fn test_hop_request() {
        let req = HopRequest {
            quantity: 20,
            original_gravity: 1020,
            hops: vec![hops::Hop {
                quantity: 10,
                alpha: 6.0,
                duration: 30,
            }],
        };

        assert_eq!(1, req.hops.len());

        let resp = req.to_response();
        assert_eq!(6.96, resp.ibu);
    }

    #[test]
    fn test_malt_deserialization() {
        let deserialized: MaltRequest = serde_json::from_str(
            "{
                \"efficiency\":75,
                \"quantity\":50,
                \"malts\":[{\"ebc\":10,\"quantity\":200}]
            }",
        )
        .unwrap();
        assert_eq!(deserialized.malts[0].ebc, 10.0);
    }

    #[test]
    fn test_hop_deserialization() {
        let deserialized: HopRequest = serde_json::from_str(
            "{
                \"quantity\":50,
                \"original_gravity\":1030,
                \"hops\":[{\"quantity\":7,\"alpha\":12,\"duration\":60}]
            }",
        )
        .unwrap();
        assert_eq!(deserialized.hops[0].alpha, 12.0);
    }

    #[test]
    fn test_malt_serialization() {
        let resp = MaltResponse {
            ebc: 10.0,
            color: malt::calculate_color(10.0),
        };

        let serialized = serde_json::to_string(&resp).unwrap();
        assert_eq!("{\"ebc\":10.0,\"color\":[222,124,0]}", serialized);
    }

    #[test]
    fn test_hop_serialization() {
        let resp = HopResponse { ibu: 40.0 };

        let serialized = serde_json::to_string(&resp).unwrap();
        assert_eq!("{\"ibu\":40.0}", serialized);
    }
}

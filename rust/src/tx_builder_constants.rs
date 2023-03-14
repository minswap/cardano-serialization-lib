use super::*;

// The first element is the cost model, which is an array of 166 operations costs, ordered by asc operaion names.
// The second value is the pre-calculated `language_views_encoding` value required for the script hash creation.
// The cost-model values are taken from the genesis block - https://github.com/input-output-hk/cardano-node/blob/master/configuration/cardano/mainnet-alonzo-genesis.json#L26-L195
// The keys on the genesis block object (operation names) are sorted plain alphabetically.

#[wasm_bindgen]
pub struct TxBuilderConstants();

#[wasm_bindgen]
impl TxBuilderConstants {
    pub fn plutus_default_cost_models() -> Costmdls {
        TxBuilderConstants::plutus_vasil_cost_models()
    }

    pub fn plutus_alonzo_cost_models() -> Costmdls {
        let mut res = Costmdls::new();
        res.insert(
            &Language::new_plutus_v1(),
            &CostModel::from(vec![
                197209, 0, 1, 1, 396231, 621, 0, 1, 150000, 1000, 0, 1, 150000, 32, 2477736, 29175,
                4, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 100,
                100, 29773, 100, 150000, 32, 150000, 32, 150000, 32, 150000, 1000, 0, 1, 150000,
                32, 150000, 1000, 0, 8, 148000, 425507, 118, 0, 1, 1, 150000, 1000, 0, 8, 150000,
                112536, 247, 1, 150000, 10000, 1, 136542, 1326, 1, 1000, 150000, 1000, 1, 150000,
                32, 150000, 32, 150000, 32, 1, 1, 150000, 1, 150000, 4, 103599, 248, 1, 103599,
                248, 1, 145276, 1366, 1, 179690, 497, 1, 150000, 32, 150000, 32, 150000, 32,
                150000, 32, 150000, 32, 150000, 32, 148000, 425507, 118, 0, 1, 1, 61516, 11218, 0,
                1, 150000, 32, 148000, 425507, 118, 0, 1, 1, 148000, 425507, 118, 0, 1, 1, 2477736,
                29175, 4, 0, 82363, 4, 150000, 5000, 0, 1, 150000, 32, 197209, 0, 1, 1, 150000, 32,
                150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 3345831, 1,
                1,
            ]),
        );
        res
    }

    pub fn plutus_vasil_cost_models() -> Costmdls {
        let mut res = Costmdls::new();
        res.insert(
            &Language::new_plutus_v1(),
            &CostModel::from(vec![
                205665, 812, 1, 1, 1000, 571, 0, 1, 1000, 24177, 4, 1, 1000, 32, 117366, 10475, 4,
                23000, 100, 23000, 100, 23000, 100, 23000, 100, 23000, 100, 23000, 100, 100, 100,
                23000, 100, 19537, 32, 175354, 32, 46417, 4, 221973, 511, 0, 1, 89141, 32, 497525,
                14068, 4, 2, 196500, 453240, 220, 0, 1, 1, 1000, 28662, 4, 2, 245000, 216773, 62,
                1, 1060367, 12586, 1, 208512, 421, 1, 187000, 1000, 52998, 1, 80436, 32, 43249, 32,
                1000, 32, 80556, 1, 57667, 4, 1000, 10, 197145, 156, 1, 197145, 156, 1, 204924,
                473, 1, 208896, 511, 1, 52467, 32, 64832, 32, 65493, 32, 22558, 32, 16563, 32,
                76511, 32, 196500, 453240, 220, 0, 1, 1, 69522, 11687, 0, 1, 60091, 32, 196500,
                453240, 220, 0, 1, 1, 196500, 453240, 220, 0, 1, 1, 806990, 30482, 4, 1927926,
                82523, 4, 265318, 0, 4, 0, 85931, 32, 205665, 812, 1, 1, 41182, 32, 212342, 32,
                31220, 32, 32696, 32, 43357, 32, 32247, 32, 38314, 32, 57996947, 18975, 10,
            ]),
        );
        res.insert(
            &Language::new_plutus_v2(),
            &CostModel::from(vec![
                205665,
                812,
                1,
                1,
                1000,
                571,
                0,
                1,
                1000,
                24177,
                4,
                1,
                1000,
                32,
                117366,
                10475,
                4,
                23000,
                100,
                23000,
                100,
                23000,
                100,
                23000,
                100,
                23000,
                100,
                23000,
                100,
                100,
                100,
                23000,
                100,
                19537,
                32,
                175354,
                32,
                46417,
                4,
                221973,
                511,
                0,
                1,
                89141,
                32,
                497525,
                14068,
                4,
                2,
                196500,
                453240,
                220,
                0,
                1,
                1,
                1000,
                28662,
                4,
                2,
                245000,
                216773,
                62,
                1,
                1060367,
                12586,
                1,
                208512,
                421,
                1,
                187000,
                1000,
                52998,
                1,
                80436,
                32,
                43249,
                32,
                1000,
                32,
                80556,
                1,
                57667,
                4,
                1000,
                10,
                197145,
                156,
                1,
                197145,
                156,
                1,
                204924,
                473,
                1,
                208896,
                511,
                1,
                52467,
                32,
                64832,
                32,
                65493,
                32,
                22558,
                32,
                16563,
                32,
                76511,
                32,
                196500,
                453240,
                220,
                0,
                1,
                1,
                69522,
                11687,
                0,
                1,
                60091,
                32,
                196500,
                453240,
                220,
                0,
                1,
                1,
                196500,
                453240,
                220,
                0,
                1,
                1,
                1159724,
                392670,
                0,
                2,
                806990,
                30482,
                4,
                1927926,
                82523,
                4,
                265318,
                0,
                4,
                0,
                85931,
                32,
                205665,
                812,
                1,
                1,
                41182,
                32,
                212342,
                32,
                31220,
                32,
                32696,
                32,
                43357,
                32,
                32247,
                32,
                38314,
                32,
                35892428,
                10,
                57996947,
                18975,
                10,
                38887044,
                32947,
                10
            ]),
        );
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::tx_builder_constants::*;

    #[test]
    pub fn cost_model_test() {
        assert_eq!(
            TxBuilderConstants::plutus_alonzo_cost_models()
                .get(&Language::new_plutus_v1())
                .unwrap()
                .len(),
            166,
        );
        assert_eq!(
            TxBuilderConstants::plutus_vasil_cost_models()
                .get(&Language::new_plutus_v1())
                .unwrap()
                .len(),
            166,
        );
        assert_eq!(
            TxBuilderConstants::plutus_vasil_cost_models()
                .get(&Language::new_plutus_v2())
                .unwrap()
                .len(),
            175,
        );
        assert_eq!(
            hex::encode(TxBuilderConstants::plutus_alonzo_cost_models().language_views_encoding()),
            "a141005901d59f1a000302590001011a00060bc719026d00011a000249f01903e800011a000249f018201a0025cea81971f70419744d186419744d186419744d186419744d186419744d186419744d18641864186419744d18641a000249f018201a000249f018201a000249f018201a000249f01903e800011a000249f018201a000249f01903e800081a000242201a00067e2318760001011a000249f01903e800081a000249f01a0001b79818f7011a000249f0192710011a0002155e19052e011903e81a000249f01903e8011a000249f018201a000249f018201a000249f0182001011a000249f0011a000249f0041a000194af18f8011a000194af18f8011a0002377c190556011a0002bdea1901f1011a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000242201a00067e23187600010119f04c192bd200011a000249f018201a000242201a00067e2318760001011a000242201a00067e2318760001011a0025cea81971f704001a000141bb041a000249f019138800011a000249f018201a000302590001011a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a00330da70101ff",
        );
        assert_eq!(
            hex::encode(TxBuilderConstants::plutus_vasil_cost_models().language_views_encoding()),
            "a20198af1a0003236119032c01011903e819023b00011903e8195e7104011903e818201a0001ca761928eb041959d818641959d818641959d818641959d818641959d818641959d81864186418641959d81864194c5118201a0002acfa182019b551041a000363151901ff00011a00015c3518201a000797751936f404021a0002ff941a0006ea7818dc0001011903e8196ff604021a0003bd081a00034ec5183e011a00102e0f19312a011a00032e801901a5011a0002da781903e819cf06011a00013a34182019a8f118201903e818201a00013aac0119e143041903e80a1a00030219189c011a00030219189c011a0003207c1901d9011a000330001901ff0119ccf3182019fd40182019ffd5182019581e18201940b318201a00012adf18201a0002ff941a0006ea7818dc0001011a00010f92192da7000119eabb18201a0002ff941a0006ea7818dc0001011a0002ff941a0006ea7818dc0001011a0011b22c1a0005fdde00021a000c504e197712041a001d6af61a0001425b041a00040c660004001a00014fab18201a0003236119032c010119a0de18201a00033d7618201979f41820197fb8182019a95d1820197df718201995aa18201b00000004a817c8001b00000004a817c8001a009063b91903fd0a1b00000004a817c800001b00000004a817c80041005901b69f1a0003236119032c01011903e819023b00011903e8195e7104011903e818201a0001ca761928eb041959d818641959d818641959d818641959d818641959d818641959d81864186418641959d81864194c5118201a0002acfa182019b551041a000363151901ff00011a00015c3518201a000797751936f404021a0002ff941a0006ea7818dc0001011903e8196ff604021a0003bd081a00034ec5183e011a00102e0f19312a011a00032e801901a5011a0002da781903e819cf06011a00013a34182019a8f118201903e818201a00013aac0119e143041903e80a1a00030219189c011a00030219189c011a0003207c1901d9011a000330001901ff0119ccf3182019fd40182019ffd5182019581e18201940b318201a00012adf18201a0002ff941a0006ea7818dc0001011a00010f92192da7000119eabb18201a0002ff941a0006ea7818dc0001011a0002ff941a0006ea7818dc0001011a000c504e197712041a001d6af61a0001425b041a00040c660004001a00014fab18201a0003236119032c010119a0de18201a00033d7618201979f41820197fb8182019a95d1820197df718201995aa18201a009063b91903fd0aff",
        );
    }
}

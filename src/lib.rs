use std::collections::HashMap;
use log::info;
use rand_chacha::rand_core::SeedableRng;
use crate::data::point::Point;
use crate::okvs::Encoder;

pub mod data;
pub mod hash;
pub mod okvs;
pub mod sub_protocol;

#[derive(Debug)]
pub enum OKVSEncoder {
    LagrangePolynomial,
}

#[derive(Debug)]
pub enum SubProtocol {
    Insecure,
}

pub struct FuzzyPSI {
    h1: u64,
    h2: u64,
    threshold: u64,
    items_party_a: Vec<u64>,
    items_party_b: Vec<u64>,
    encoder: OKVSEncoder,
    sub_protocol: SubProtocol,
}

impl FuzzyPSI {
    pub fn new() -> Self {
        Self {
            h1: 17,
            h2: 23,
            threshold: 10,
            items_party_a: Vec::new(),
            items_party_b: Vec::new(),
            encoder: OKVSEncoder::LagrangePolynomial,
            sub_protocol: SubProtocol::Insecure,
        }
    }

    pub fn with_h1(mut self, h1: u64) -> Self {
        self.h1 = h1;
        self
    }

    pub fn with_h2(mut self, h2: u64) -> Self {
        self.h2 = h2;
        self
    }

    pub fn with_threshold(mut self, threshold: u64) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn with_items_party_a(mut self, items: Vec<u64>) -> Self {
        self.items_party_a = items;
        self
    }

    pub fn with_items_party_b(mut self, items: Vec<u64>) -> Self {
        self.items_party_b = items;
        self
    }

    pub fn with_encoder(mut self, encoder: OKVSEncoder) -> Self {
        self.encoder = encoder;
        self
    }

    pub fn with_sub_protocol(mut self, sub_protocol: SubProtocol) -> Self {
        self.sub_protocol = sub_protocol;
        self
    }

    pub fn run(&self) -> Vec<u64> {
        simple_logger::SimpleLogger::new().env().init().unwrap();
        info!("Starting FuzzyPSI with h1={}, h2={}, threshold={}", self.h1, self.h2, self.threshold);
        info!("Number of items for party A: {}", self.items_party_a.len());
        info!("Number of items for party B: {}", self.items_party_b.len());
        info!("Encoder: {:?}", self.encoder);
        info!("Sub-protocol: {:?}", self.sub_protocol);

        let mut rnd = rand_chacha::ChaCha20Rng::from_os_rng();
        let mut states = HashMap::new();

        let encoder = match self.encoder {
            OKVSEncoder::LagrangePolynomial => okvs::lagrange_polynomial_okvs::LagrangePolynomialOKVS::encode
        };

        // Step 1, party A; in the end, an OKVS is created and sent to party B
        let mut transmit_from_a = Vec::new();
        for bin in hash::create_bins_h1(&self.items_party_a, self.threshold) {
            // throw an error if more than 1 item hashes to the same bin.
            let orig_point = hash::invert_bin(bin, &self.items_party_a, self.threshold);
            let (message, state) = sub_protocol::insecure::sub_protocol_1(orig_point);
            states.insert(bin, state);
            transmit_from_a.push(Point::new(bin, message));
        }

        todo!()
        //
        // const h_1: usize = 17;
        // const h_2: usize = 23;
        // const distance_threshold: u64 = 10;
        //
        // let items_party_a = vec![5u64];
        // let mut states = HashMap::new();
        // let items_party_b = vec![5u64];
        //

        //
        // // fill up the list of elements with random noise to match |A| * h_1 + 1
        // let target_elements_a = transmit_from_a.len() * h_1 + 1;
        // assert!(
        //     target_elements_a < 500,
        //     "Target elements A must be less than 500, otherwise encoding will take ages."
        // );
        // while transmit_from_a.len() < target_elements_a {
        //     transmit_from_a.push(Point::new(rnd.next_u64(), rnd.next_u64()));
        // }
        //
        // let binding = [transmit_from_a];
        // let okvs_a = fuzzy_psi::okvs::LagrangePolynomialOKVS::encode(&binding);
        //
        // // Step 2, party B; in the end, an OKVS is created and sent back to party A
        // let mut transmit_from_b = Vec::new();
        // for bin in fuzzy_psi::hash::create_bins_h2(&items_party_b, distance_threshold) {
        //     let message_from_a = okvs_a.decode(bin)[0];
        //     let orig_point = fuzzy_psi::hash::invert_bin(bin, &items_party_b, distance_threshold);
        //     let message_from_b = fuzzy_psi::sub_protocol::sub_protocol_2(message_from_a, orig_point);
        //     transmit_from_b.push(Point::new(bin, message_from_b));
        // }
        //
        // // fill up the list of elements with random noise to match |B| * h_2 + 1
        // let target_elements_b = transmit_from_b.len() * h_2 + 1;
        // assert!(
        //     target_elements_b < 500,
        //     "Target elements B must be less than 500, otherwise encoding will take ages."
        // );
        // while transmit_from_b.len() < target_elements_b {
        //     transmit_from_b.push(Point::new(rnd.next_u64(), rnd.next_u64()));
        // }
        //
        // let binding = [transmit_from_b];
        // let okvs_b = fuzzy_psi::okvs::LagrangePolynomialOKVS::encode(&binding);
        //
        // // Step 3, party A; in the end, the intersection is calculated
        // for bin in fuzzy_psi::hash::create_bins_h1(&items_party_a, distance_threshold) {
        //     let message_from_b = okvs_b.decode(bin)[0];
        //     let state = *states.get(&bin).unwrap();
        //     if let Some(p) =
        //         fuzzy_psi::sub_protocol::sub_protocol_3(state, message_from_b, distance_threshold)
        //     {
        //         println!("Intersection: {}", p);
        //     }
        // }
    }
}

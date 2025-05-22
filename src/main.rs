use crate::okvs::near_optimal::okvs::{Okvs, OkvsKey, OkvsValue, RbOkvs};
use rand::SeedableRng;
use rand_chacha::rand_core::RngCore;
use std::collections::HashMap;

mod hash;
mod okvs;

const MIN_OKVS_LENGTH: usize = 64;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Execute Fuzzy PSI example");

    let mut rng = rand_chacha::ChaCha20Rng::from_os_rng();
    let distance_threshold = 2;
    let h_1 = 1;
    let h_2 = 2;

    let items_party_a = vec![1u64, 10u64, 100u64, 1000u64, 10000u64];
    let items_party_b = vec![1u64, 1000u64, 100u64];

    // --------------------------------------------------------------------

    // First part: Party A creates the bins and stores them into the OKVS.
    let okvs_a_length = core::cmp::max(items_party_a.len() * h_1 + 1, MIN_OKVS_LENGTH);
    let okvs_a = RbOkvs::new(okvs_a_length);
    let mut s_a = Vec::new();
    let mut state_a = HashMap::new();
    for bin in hash::create_bins(&items_party_a, distance_threshold) {
        // execute subprot 1
        let inv = hash::invert_bin(bin, &items_party_a, distance_threshold);
        if inv.is_empty() {
            continue;
        }
        let m_b = *inv.first().unwrap();
        state_a.insert(bin, m_b);
        s_a.push((OkvsKey(bin.to_le_bytes()), OkvsValue(m_b.to_le_bytes())))
    }
    while s_a.len() < okvs_a_length {
        s_a.push((
            OkvsKey(rng.next_u64().to_le_bytes()),
            OkvsValue(rng.next_u64().to_le_bytes()),
        ));
    }
    let enc_a = okvs_a.encode(s_a)?;

    // Part two: Party B receives the encoding, creates its own bins and
    // decodes the values from party A. Then, it calculates its own message part for the bin.
    let okvs_b_length = core::cmp::max(items_party_b.len() * h_2 + 1, MIN_OKVS_LENGTH);
    let okvs_b = RbOkvs::new(okvs_b_length);
    let mut s_b = Vec::new();
    for bin in hash::create_bins(&items_party_b, distance_threshold) {
        let bin_key = OkvsKey(bin.to_le_bytes());
        let m_1 = okvs_a.decode(&enc_a, &bin_key);

        // execute prot 1
        let inv = hash::invert_bin(bin, &items_party_b, distance_threshold);
        if inv.is_empty() {
            continue;
        }
        let val_from_a = u64::from_ne_bytes(m_1.0);
        let point = *inv.first().unwrap();
        // "subprotocol 2": check if they are "equal", if yes, add the point with the bin to
        // the okvs
        if val_from_a == point {
            let m_b = point;
            s_b.push((OkvsKey(bin.to_le_bytes()), OkvsValue(m_b.to_le_bytes())))
        }
    }
    while s_b.len() < okvs_b_length {
        s_b.push((
            OkvsKey(rng.next_u64().to_le_bytes()),
            OkvsValue(rng.next_u64().to_le_bytes()),
        ));
    }
    let enc_b = okvs_b.encode(s_b)?;

    // Part three: party A receives the encoding from party B and decodes the values (with
    // the bins) and then determines if the values are close enough.
    let mut intersection = Vec::new();
    for bin in hash::create_bins(&items_party_a, distance_threshold) {
        let bin_key = OkvsKey(bin.to_le_bytes());
        let m_2 = okvs_b.decode(&enc_b, &bin_key);

        // execute subprot 3
        let state_val = state_a.get(&bin);
        if state_val.is_none() {
            continue;
        }
        let state_val = *state_val.unwrap();
        let val_from_b = u64::from_ne_bytes(m_2.0);
        if state_val.abs_diff(val_from_b) <= distance_threshold {
            intersection.push(val_from_b);
        }
    }

    println!("Intersection: {:?}", intersection);

    Ok(())
}

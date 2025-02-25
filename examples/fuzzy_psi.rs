use fuzzy_psi::data::point::Point;
use fuzzy_psi::okvs::{Encoder, Store};
use rand_chacha::rand_core::RngCore;
use rand_chacha::rand_core::SeedableRng;
use std::collections::HashMap;
use fuzzy_psi::FuzzyPSI;

/// Execute the Fuzzy PSI (Private Set Intersection) example.
///
/// "Step 1", party A creates bins,
/// for each bin in the list, execute subprotocol 1
/// to get a message and a state for the bin.
/// Add the bin and the message to a set.
/// Encode the set in an OKVS.
/// Send the OKVS to party B.
///
/// "Step 2", party B receives the OKVS from party A.
/// B creates its own bins. For each bin, decode
/// the message from the OKVS and execute subprotocol 2.
/// Add the result to a set.
/// Encode the resulting set in an OKVS.
/// Send the OKVS to party A.
///
/// "Step 3", party A receives the OKVS from party B.
/// For each bin, decode the message from B's OKVS.
/// Execute subprotocol 3 to see which items are within
/// the distance threshold. Add the result to a set.
///
/// The result is the intersection of the items
/// that are within the distance threshold.
fn main() {
    FuzzyPSI::new()
        .with_h1(17)
        .with_h2(23)
        .with_threshold(10)
        .with_items_party_a(vec![5])
        .with_items_party_b(vec![5])
        .run();


    // println!("Execute Fuzzy PSI example");
    // let mut rnd = rand_chacha::ChaCha20Rng::from_os_rng();
    //
    // const h_1: usize = 17;
    // const h_2: usize = 23;
    // const distance_threshold: u64 = 10;
    //
    // let items_party_a = vec![5u64];
    // let mut states = HashMap::new();
    // let items_party_b = vec![5u64];
    //
    // // Step 1, party A; in the end, an OKVS is created and sent to party B
    // let mut transmit_from_a = Vec::new();
    // for bin in fuzzy_psi::hash::create_bins_h1(&items_party_a, distance_threshold) {
    //     // throw an error if more than 1 item hashes to the same bin.
    //     let orig_point = fuzzy_psi::hash::invert_bin(bin, &items_party_a, distance_threshold);
    //     let (message, state) = fuzzy_psi::sub_protocol::sub_protocol_1(orig_point);
    //     states.insert(bin, state);
    //     transmit_from_a.push(Point::new(bin, message));
    // }
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

    // h1: okvs add random stuff length of A * h1 + 1
    // h1: use constant. s.t. the OKVS encode still works (which means < 500)
    // h2: same thing as h1

    // transmit P
    // for B:
    /*
      create bins for all points in B
      decode the value (message) from A with the bin
      H_2^-1 -> like H_1^-1, give the point that created the bin
      PROT2: Message1, and my point (of b) -> create message 2
      essentially: also just give back the point (insecure one)
      send back Q (okvs)

      step 3:
      for each of the bins in A, decode the message from B (with the b)
      PROT3: check if the message and the "state" (both the actual points) are close enough (e.g. delta)
    */
}

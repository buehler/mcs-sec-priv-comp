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
    println!("Execute Fuzzy PSI example");

    let distance_threshold = 2;
    let items_party_a = vec![1u64];
    let items_party_b = vec![1u64];

    for bin in fuzzy_psi::hash::create_bins(&items_party_a, distance_threshold){
        // execute prot 1

    }
}

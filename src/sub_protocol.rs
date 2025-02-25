pub mod insecure {
    pub fn sub_protocol_1(point: u64) -> (u64, u64) {
        // insecure protocol, just return the point.
        (point, point)
    }

    pub fn sub_protocol_2(_msg_from_a: u64, point: u64) -> u64 {
        // insecure protocol, just return the point.
        point
    }

    pub fn sub_protocol_3(state: u64, msg_from_b: u64, delta: u64) -> Option<u64> {
        if ((if state > delta { state - delta } else { 0 })..=(state + delta)).contains(&msg_from_b)
        {
            Some(msg_from_b)
        } else {
            None
        }
    }
}

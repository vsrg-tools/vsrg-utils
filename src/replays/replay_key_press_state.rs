macro_rules! bitmask_to_vec {
    ($inst:expr, $($bit:expr => $repr:expr),*) => {
        {
            let mut lanes: Vec<i32> = Vec::new();

            $(
                if $inst.contains($bit) {
                    lanes.push($repr);
                }
            )*

            lanes
        }
    };
}

bitflags! {
    pub struct ReplayKeyPressState: i16 {
        const K1 = 1 << 0;
        const K2 = 1 << 1;
        const K3 = 1 << 2;
        const K4 = 1 << 3;
        const K5 = 1 << 4;
        const K6 = 1 << 5;
        const K7 = 1 << 6;
        const K8 = 1 << 7;
        const K9 = 1 << 8;
    }
}

impl ReplayKeyPressState {
    pub fn to_lanes(&self) -> Vec<i32> {
        bitmask_to_vec!(
            self,
            ReplayKeyPressState::K1 => 0,
            ReplayKeyPressState::K2 => 1,
            ReplayKeyPressState::K3 => 2,
            ReplayKeyPressState::K4 => 3,
            ReplayKeyPressState::K5 => 4,
            ReplayKeyPressState::K6 => 5,
            ReplayKeyPressState::K7 => 6,
            ReplayKeyPressState::K8 => 7,
            ReplayKeyPressState::K9 => 8
        )
    }
}

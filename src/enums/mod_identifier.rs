use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct ModIdentifier: i64 {
        const None = -1;
        const NoSliderVelocity = 1 << 0;
        const Speed05X = 1 << 1;
        const Speed06X = 1 << 2;
        const Speed07X = 1 << 3;
        const Speed08X = 1 << 4;
        const Speed09X = 1 << 5;
        const Speed11X = 1 << 6;
        const Speed12X = 1 << 7;
        const Speed13X = 1 << 8;
        const Speed14X = 1 << 9;
        const Speed15X = 1 << 10;
        const Speed16X = 1 << 11;
        const Speed17X = 1 << 12;
        const Speed18X = 1 << 13;
        const Speed19X = 1 << 14;
        const Speed20X = 1 << 15;
        const Strict = 1 << 16;
        const Chill = 1 << 17;
        const NoPause = 1 << 18;
        const Autoplay = 1 << 19;
        const Paused = 1 << 20;
        const NoFail = 1 << 21;
        const NoongNotes = 1 << 22;
        const Randomize = 1 << 23;
        const Speed055X = 1 << 24;
        const Speed065X = 1 << 25;
        const Speed075X = 1 << 26;
        const Speed085X = 1 << 27;
        const Speed095X = 1 << 28;
        const Inverse = 1 << 29;
        const FullN = 1 << 30;
        const Mirror = 1 << 31;
        const Coop = 1 << 32;
        const Speed105X = 1 << 33;
        const Speed115X = 1 << 34;
        const Speed125X = 1 << 35;
        const Speed135X = 1 << 36;
        const Speed145X = 1 << 37;
        const Speed155X = 1 << 38;
        const Speed165X = 1 << 39;
        const Speed175X = 1 << 40;
        const Speed185X = 1 << 41;
        const Speed195X = 1 << 42;
        const HeatlthAdjust = 1 << 43;
        const NoMiss = 1 << 44;
    }
}

impl ModIdentifier {
    pub fn get_rate_from_mods(mods: ModIdentifier) -> f32 {
        let mut rate = 1.0f32;

        if mods.contains(ModIdentifier::None) {
            rate = 1.0
        } else if mods.contains(ModIdentifier::Speed05X) {
            rate = 0.5
        } else if mods.contains(ModIdentifier::Speed055X) {
            rate = 0.55
        } else if mods.contains(ModIdentifier::Speed06X) {
            rate = 0.6
        } else if mods.contains(ModIdentifier::Speed065X) {
            rate = 0.65
        } else if mods.contains(ModIdentifier::Speed07X) {
            rate = 0.7
        } else if mods.contains(ModIdentifier::Speed075X) {
            rate = 0.75
        } else if mods.contains(ModIdentifier::Speed08X) {
            rate = 0.8
        } else if mods.contains(ModIdentifier::Speed085X) {
            rate = 0.85
        } else if mods.contains(ModIdentifier::Speed09X) {
            rate = 0.9
        } else if mods.contains(ModIdentifier::Speed095X) {
            rate = 0.95
        } else if mods.contains(ModIdentifier::Speed105X) {
            rate = 1.05
        } else if mods.contains(ModIdentifier::Speed11X) {
            rate = 1.1
        } else if mods.contains(ModIdentifier::Speed115X) {
            rate = 1.15
        } else if mods.contains(ModIdentifier::Speed12X) {
            rate = 1.2
        } else if mods.contains(ModIdentifier::Speed125X) {
            rate = 1.25
        } else if mods.contains(ModIdentifier::Speed13X) {
            rate = 1.3
        } else if mods.contains(ModIdentifier::Speed135X) {
            rate = 1.35
        } else if mods.contains(ModIdentifier::Speed14X) {
            rate = 1.4
        } else if mods.contains(ModIdentifier::Speed145X) {
            rate = 1.45
        } else if mods.contains(ModIdentifier::Speed15X) {
            rate = 1.5
        } else if mods.contains(ModIdentifier::Speed155X) {
            rate = 1.55
        } else if mods.contains(ModIdentifier::Speed16X) {
            rate = 1.6
        } else if mods.contains(ModIdentifier::Speed165X) {
            rate = 1.65
        } else if mods.contains(ModIdentifier::Speed17X) {
            rate = 1.7
        } else if mods.contains(ModIdentifier::Speed175X) {
            rate = 1.75
        } else if mods.contains(ModIdentifier::Speed18X) {
            rate = 1.8
        } else if mods.contains(ModIdentifier::Speed185X) {
            rate = 1.85
        } else if mods.contains(ModIdentifier::Speed19X) {
            rate = 1.9
        } else if mods.contains(ModIdentifier::Speed195X) {
            rate = 1.95
        } else if mods.contains(ModIdentifier::Speed20X) {
            rate = 2.0
        }

        rate
    }
}

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

#![allow(non_upper_case_globals)]

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
        const NoLongNotes = 1 << 22;
        const Randomize = 1 << 23;
        const Speed055X = 1 << 24;
        const Speed065X = 1 << 25;
        const Speed075X = 1 << 26;
        const Speed085X = 1 << 27;
        const Speed095X = 1 << 28;
        const Inverse = 1 << 29;
        const FullLN = 1 << 30;
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
        if mods.contains(ModIdentifier::Speed05X) {
            0.5
        } else if mods.contains(ModIdentifier::Speed055X) {
            0.55
        } else if mods.contains(ModIdentifier::Speed06X) {
            0.6
        } else if mods.contains(ModIdentifier::Speed065X) {
            0.65
        } else if mods.contains(ModIdentifier::Speed07X) {
            0.7
        } else if mods.contains(ModIdentifier::Speed075X) {
            0.75
        } else if mods.contains(ModIdentifier::Speed08X) {
            0.8
        } else if mods.contains(ModIdentifier::Speed085X) {
            0.85
        } else if mods.contains(ModIdentifier::Speed09X) {
            0.9
        } else if mods.contains(ModIdentifier::Speed095X) {
            0.95
        } else if mods.contains(ModIdentifier::Speed105X) {
            1.05
        } else if mods.contains(ModIdentifier::Speed11X) {
            1.1
        } else if mods.contains(ModIdentifier::Speed115X) {
            1.15
        } else if mods.contains(ModIdentifier::Speed12X) {
            1.2
        } else if mods.contains(ModIdentifier::Speed125X) {
            1.25
        } else if mods.contains(ModIdentifier::Speed13X) {
            1.3
        } else if mods.contains(ModIdentifier::Speed135X) {
            1.35
        } else if mods.contains(ModIdentifier::Speed14X) {
            1.4
        } else if mods.contains(ModIdentifier::Speed145X) {
            1.45
        } else if mods.contains(ModIdentifier::Speed15X) {
            1.5
        } else if mods.contains(ModIdentifier::Speed155X) {
            1.55
        } else if mods.contains(ModIdentifier::Speed16X) {
            1.6
        } else if mods.contains(ModIdentifier::Speed165X) {
            1.65
        } else if mods.contains(ModIdentifier::Speed17X) {
            1.7
        } else if mods.contains(ModIdentifier::Speed175X) {
            1.75
        } else if mods.contains(ModIdentifier::Speed18X) {
            1.8
        } else if mods.contains(ModIdentifier::Speed185X) {
            1.85
        } else if mods.contains(ModIdentifier::Speed19X) {
            1.9
        } else if mods.contains(ModIdentifier::Speed195X) {
            1.95
        } else if mods.contains(ModIdentifier::Speed20X) {
            2.0
        } else {
            1.0
        }
    }
}

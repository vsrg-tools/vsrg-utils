#![allow(non_upper_case_globals)]

bitflags! {
    #[derive(Default)]
    pub struct QssPatternFlags: i64 {
        const Unknown = 0;
        const MiniJack = 1 << 0;
        const ChordJack = 1 << 1;
        const KoreaJack = 1 << 2;
        const LongJack = 1 << 3;
        const QuadJack = 1 << 4;
        const Rolls = 1 << 5;
        const LightStream = 1 << 6;
        const JumpStream = 1 << 7;
        const HandStream = 1 << 8;
        const QuadStream = 1 << 9;
        const InverseLN = 1 << 10;
        const ReleaseLN = 1 << 11;
        const Polyrhythm = 1 << 12;
        const JumpTrill = 1 << 13;
        const SplitTrill = 1 << 14;
        const SimpleVibro = 1 << 15;
        const ControlVibro = 1 << 16;
    }
}

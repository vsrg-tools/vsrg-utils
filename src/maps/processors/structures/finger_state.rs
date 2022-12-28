use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct FingerState: u8 {
        const None = 0;
        const Index = 1 << 0;
        const Middle = 1 << 1;
        const Ring = 1 << 2;
        const Pinkie = 1 << 3;
        const Thumb = 1 << 4;
    }
}

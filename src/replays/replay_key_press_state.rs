use bitflags::bitflags;

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

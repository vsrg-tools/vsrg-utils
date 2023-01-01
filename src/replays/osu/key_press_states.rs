bitflags! {
    #[derive(Default)]
    pub struct KeyPressState: u32 {
        const K1 = 1 << 0;
        const K2 = 1 << 1;
        const K3 = 1 << 2;
        const K4 = 1 << 3;
        const K5 = 1 << 4;
        const K6 = 1 << 5;
        const K7 = 1 << 6;
        const K8 = 1 << 7;
        const K9 = 1 << 8;
        const K10 = 1 << 9;
        const K11 = 1 << 10;
        const K12 = 1 << 11;
        const K13 = 1 << 12;
        const K14 = 1 << 13;
        const K15 = 1 << 14;
        const K16 = 1 << 15;
        const K17 = 1 << 16;
        const K18 = 1 << 17;
    }
}

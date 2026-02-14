use imxrt_rt::{Family, RuntimeBuilder};

fn main() {
    // MIMXRT1060-EVK 板载 8MB QuadSPI Flash
    RuntimeBuilder::from_flexspi(Family::Imxrt1060, 8 * 1024 * 1024)
        .build()
        .unwrap();
}

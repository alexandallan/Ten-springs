use imxrt_rt::{Family, RuntimeBuilder};

fn main() {
    // MIMXRT1060-EVK 板载 8MB QuadSPI Flash
    RuntimeBuilder::from_flexspi(Family::Imxrt1060, 8 * 1024 * 1024)
        .build()
        .unwrap();

    // defmt 启用时，引入 defmt.x 链接脚本
    if std::env::var("CARGO_FEATURE_DEFMT_LOGGING").is_ok() {
        println!("cargo:rustc-link-arg=-Tdefmt.x");
    }
}

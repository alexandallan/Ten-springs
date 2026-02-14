#![no_std]
#![no_main]

// defmt 日志（feature = "defmt-logging" 时启用）
#[cfg(feature = "defmt-logging")]
use defmt_rtt as _;
#[cfg(feature = "defmt-logging")]
use panic_probe as _;

// 无日志时使用 panic-halt
#[cfg(not(feature = "defmt-logging"))]
use panic_halt as _;

use imxrt_rt::entry;
use imxrt_hal as hal;
use imxrt_ral as ral;

// 引入 FCB（FlexSPI 启动配置块），确保链接器将其包含在固件中
#[cfg(target_arch = "arm")]
use imxrt1060evk_fcb as _;

#[entry]
fn main() -> ! {
    // 1. 逐个获取外设实例（imxrt-ral v0.5 没有全局 take()）
    let mut ccm = unsafe { ral::ccm::CCM::instance() };
    let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
    let gpio1 = unsafe { ral::gpio::GPIO1::instance() };

    #[cfg(feature = "defmt-logging")]
    defmt::info!("MIMXRT1060-EVK 启动");

    // 2. 使能 GPIO1 时钟门控（不开时钟，GPIO 不工作）
    hal::ccm::clock_gate::gpio::<1>().set(&mut ccm, hal::ccm::clock_gate::ON);

    #[cfg(feature = "defmt-logging")]
    defmt::info!("GPIO1 时钟门控已使能");

    // 3. 初始化 IOMUXC，获取引脚对象
    let pads = hal::iomuxc::into_pads(iomuxc);

    // 4. 配置 LED 引脚
    //    MIMXRT1060-EVK 上用户 LED 连接在 GPIO_AD_B0_09（GPIO1 pin 9）
    let mut gpio1_port = hal::gpio::Port::new(gpio1);
    let led = gpio1_port.output(pads.gpio_ad_b0.p09);

    #[cfg(feature = "defmt-logging")]
    defmt::info!("LED (GPIO_AD_B0_09) 已配置为输出");

    // 5. 点灯主循环（约 0.5 秒间隔）
    let mut count: u32 = 0;
    loop {
        led.toggle();
        count = count.wrapping_add(1);

        #[cfg(feature = "defmt-logging")]
        defmt::debug!("LED toggle #{}", count);

        // Cortex-M7 @ 600MHz，每个循环约 1 个时钟周期
        // 300_000_000 次循环 ≈ 0.5 秒
        cortex_m::asm::delay(300_000_000);
    }
}

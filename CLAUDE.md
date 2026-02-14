# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概览

面向 **NXP MIMXRT1060-EVK** 开发板的裸机 `#![no_std]` LED 闪烁示例，使用现代 imxrt Rust 生态。

- MCU：**MIMXRT1062DVL6A**（Arm Cortex-M7 @ 600MHz，1MB SRAM，10x10mm BGA）
- 板载 Flash：HyperFlash (S26KS512) + **QSPI Flash (IS25WP064A, 8MB)**
- 用户 LED：**D18 (绿色)** → R316 (220Ω) → **GPIO_AD_B0_09**（GPIO1 pin 9）

## 构建与烧录

```bash
cargo build --release      # Release 编译
cargo run --release        # 编译并通过 probe-rs 烧录
```

### 工具链前提

- `rustup target add thumbv7em-none-eabihf`
- 烧录/调试工具：`probe-rs`（probe-rs 芯片名称为 `MIMXRT1060`）

### 开发板设置

- **SW7 拨码开关**：`OFF-OFF-ON-OFF`（QSPI NOR Flash 启动模式）
- **调试接口**：J21（20-pin ARM JTAG），SWD 最小接线：Pin 7 (SWDIO) + Pin 9 (SWCLK) + Pin 4 (GND)

## 代码架构

| 文件 | 作用 |
|------|------|
| `src/main.rs` | 入口（`#[entry]`）。获取 RAL 外设实例 → 使能时钟门控 → 配置 IOMUXC → GPIO 输出 → toggle 循环 |
| `build.rs` | 调用 `RuntimeBuilder::from_flexspi()` 生成 `imxrt-link.x` 链接脚本 |
| `.cargo/config.toml` | 编译目标、probe-rs runner、**链接脚本指定**（`-Timxrt-link.x -Tdevice.x`） |

## Crate 分工与 Feature 配置

| Crate | Feature | 作用 |
|-------|---------|------|
| `imxrt-ral` 0.5 | `imxrt1062`, **`rt`** | 寄存器访问层 + 中断向量表 |
| `imxrt-hal` 0.5 | `imxrt1060` | 硬件抽象层（GPIO、IOMUXC、CCM 等） |
| `imxrt-rt` 0.1 | `device` | 启动运行时（需在 `[dependencies]` 和 `[build-dependencies]` 中同时声明） |
| `imxrt1060evk-fcb` 0.1 | — | EVK 板载 QSPI Flash 的 FlexSPI 配置块 |
| `cortex-m` 0.7 | — | Cortex-M 核心支持 |
| `panic-halt` 0.2 | — | Panic 处理（CPU 挂起） |

## 链接配置要点（关键踩坑记录）

构建一个可烧录的 imxrt 固件需要三个环节配合：

1. **`build.rs`** 调用 `RuntimeBuilder` 生成 `imxrt-link.x`（含 MEMORY 定义、`.boot` 段、FlexSPI 启动头）
2. **`.cargo/config.toml`** 必须显式指定两个链接脚本：
   ```toml
   rustflags = ["-C", "link-arg=-Timxrt-link.x", "-C", "link-arg=-Tdevice.x"]
   ```
   - `-Timxrt-link.x`：替代 cortex-m-rt 默认的 `link.x`，提供 imxrt 专用内存布局
   - `-Tdevice.x`：由 `imxrt-ral`（需 `rt` feature）生成，提供中断处理函数的弱符号默认实现
3. **`imxrt-ral` 必须启用 `rt` feature**：否则不会编译 `__INTERRUPTS` 符号和 `device.x`

### 固件内存映射

| 段 | 加载地址 (LMA) | 运行地址 (VMA) | 存储区域 |
|----|---------------|---------------|----------|
| `.boot`（FCB + IVT） | `0x60000000` | `0x60000000` | Flash（XIP） |
| `.vector_table` | `0x60002000` | `0x20002000` | Flash → DTCM |
| `.text` | `0x600022B8` | `0x00000000` | Flash → ITCM |
| `.rodata` / `.data` | Flash | OCRAM (`0x20200000`) | Flash → OCRAM |

## imxrt-ral v0.5 API 要点

- **没有** 全局 `ral::take()`，每个外设需单独获取：`unsafe { ral::gpio::GPIO1::instance() }`
- 多实例外设各有独立类型：`GPIO1`、`GPIO2`……
- RAL 不提供资源管理策略，安全性由调用者负责（只在 `main()` 顶部调用一次）

## GPIO 编程模式（参考手册 Chapter 12）

1. 通过 IOMUXC 将引脚配置为 GPIO 模式（HAL: `into_pads()` + `Port::output()`）
2. **必须先使能时钟门控**：`hal::ccm::clock_gate::gpio::<N>().set(&mut ccm, ON)`
3. `Output` 方法：`.set()` / `.clear()` / `.toggle()` / `.is_set()`（均为 `&self`）

## 硬件参考

详细硬件信息参见 [MIMXRT1060-EVK-硬件参考.md](./MIMXRT1060-EVK-硬件参考.md)。

## 注意事项

- 链接脚本由 `imxrt-rt` 的 `RuntimeBuilder` 生成，不需要本地 `memory.x`
- FCB 由 `imxrt1060evk-fcb` crate 提供，不需要手写
- probe-rs 芯片名称是 `MIMXRT1060`（不是完整型号 `MIMXRT1062DVL6A`）
- 源码注释使用中文（简体）
- 交互请使用中文

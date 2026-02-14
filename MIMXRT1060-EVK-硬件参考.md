# MIMXRT1060-EVK 硬件参考

基于 `MIMXRT1060-EVK-资料/` 中的原理图、数据表、参考手册及应用笔记整理。

## MCU 核心参数

| 项目 | 规格 |
|------|------|
| 型号 | MIMXRT1062DVL6A |
| 内核 | Arm Cortex-M7 @ 600 MHz，FPU (VFPv5) |
| Cache | 32 KB I-Cache + 32 KB D-Cache |
| 片上 RAM | 512 KB（可灵活分配为 ITCM / DTCM / OCRAM）+ 512 KB OCRAM |
| Boot ROM | 128 KB |
| 封装 | 196-pin MAPBGA, 10×10 mm, 0.65 mm pitch |
| GPIO | 127 个（其中 124 个紧耦合，运行在与内核相同的频率） |

## 开发板关键硬件连接

### 用户 LED（原理图 Sheet 4）

```
GPIO_AD_B0_09 (F14) ──→ R316 (220Ω) ──→ D18 (绿色 LED) ──→ GND
```

- GPIO 端口：**GPIO1**，bit 位：**9**
- HAL 引脚：`pads.gpio_ad_b0.p09`
- LED 低电平点亮（GPIO 输出高 → LED 亮）

### 板载存储

| 器件 | 型号 | 容量 | 接口 |
|------|------|------|------|
| QSPI Flash | ISSI IS25WP064A | 8 MB | FlexSPI (Quad) |
| HyperFlash | Cypress S26KS512 | 64 MB | FlexSPI (Octal) |
| SDRAM | ISSI IS42S16160J | 32 MB | SEMC (16-bit) |

### 启动模式拨码开关 SW7

| SW7-1 | SW7-2 | SW7-3 | SW7-4 | 启动设备 |
|-------|-------|-------|-------|----------|
| OFF | ON | ON | OFF | HyperFlash |
| OFF | OFF | ON | OFF | **QSPI NOR Flash** |
| ON | OFF | ON | OFF | SD Card |

- SW7 对应 BOOT_MODE 和 BOOT_CFG 引脚
- BOOT_MODE[1:0] = 10b 为 Internal Boot（正常启动模式）
- BOOT_MODE[1:0] = 01b 为 Serial Downloader（下载模式）

### JTAG/SWD 调试接口 J21（原理图 Sheet 12）

J21 为标准 ARM 20-pin JTAG 接口（2×10，0.05" 间距，BH25420B301）：

```
        ┌───────────┐
  VTref │ 1       2 │ VCC (JTAG_PWR)
 nTRST  │ 3       4 │ GND
    TDI │ 5       6 │ GND
    TMS │ 7       8 │ GND    ← Pin 7 = SWDIO
    TCK │ 9      10 │ GND    ← Pin 9 = SWCLK
   RTCK │11      12 │ GND
    TDO │13      14 │ GND    ← Pin 13 = SWO
 nSRST  │15      16 │ GND
  DBGRQ │17      18 │ GND
 DBGACK │19      20 │ GND
        └───────────┘
```

**SWD 最小接线（3 线）：**

| DAP 调试器 | J21 引脚 | 信号 |
|-----------|---------|------|
| SWDIO | **Pin 7** | 数据线 |
| SWCLK | **Pin 9** | 时钟线 |
| GND | **Pin 4**（或任意偶数脚） | 地线 |

推荐额外连接：Pin 1 (VTref) 和 Pin 15 (nRESET)。

> **注意**：Pin 5 是 TDI（JTAG 专用），不能用于 SWD 模式。

### 其它关键接口

| 功能 | 引脚/器件 | 备注 |
|------|-----------|------|
| 用户按钮 | SW8 (USER_BUTTON) | |
| 复位按钮 | SW3 (POR_BUTTON) | |
| Debug USB | J41 (OpenSDA) | 同时提供串口和 DAP-Link 调试 |
| USB OTG | J9 | USB OTG1 |
| 以太网 | KSZ8081 (RMII) | 100Base-T |
| Audio Codec | WM8960 | I2S 接口 |
| 运动传感器 | FXOS8700CQ | I2C 接口 |
| Camera | OV7725 (CSI) | |
| LCD | 4.3" TFT 480×272 | 并行 RGB 接口 |

## FlexSPI 启动流程（参考手册 Chapter 9 + AN12108）

```
POR 复位
  │
  ▼
Boot ROM 读取 BOOT_MODE 引脚
  │
  ▼ (Internal Boot)
从 FlexSPI Flash 偏移 0x0 读取 512 字节 FCB
  │  （FlexSPI Configuration Block，tag = 0x42464346 "FCFB"）
  │  （使用 BOOT_CFG2[2:0] 指定的读命令，时钟 30MHz）
  ▼
根据 FCB 配置 FlexSPI 控制器
  │
  ▼
读取 IVT（Image Vector Table，偏移 0x1000）
  │  IVT tag = 0xD1
  ▼
读取 Boot Data → 获取镜像起始地址和大小
  │
  ▼
（可选）执行 DCD 初始化 SDRAM 等外设
  │
  ▼
HAB 验证（安全启动）
  │
  ▼
跳转到用户程序入口
```

### Flash 镜像内存布局

| 偏移 | 内容 | 大小 |
|------|------|------|
| 0x0000 | FCB (FlexSPI Configuration Block) | 512 B |
| 0x1000 | IVT (Image Vector Table) | 32 B |
| 0x1020 | Boot Data | 12 B |
| 0x1030 | DCD (Device Configuration Data，可选) | 可变 |
| 0x2000 | 用户程序起始 | — |

### QSPI Flash 启动要点（AN12108）

- DQS 引脚需保持**悬空**以支持 133MHz 读写频率（不悬空仅支持 60MHz）
- FCB 中的 LUT 需配置 Quad Read (0xEB)、Read Status (0x05)、Write Enable (0x06) 等命令
- `imxrt1060evk-fcb` crate 已包含针对 IS25WP Flash 的完整 FCB 配置

## GPIO 编程参考（参考手册 Chapter 12）

### 寄存器概览

| 寄存器 | 偏移 | 功能 |
|--------|------|------|
| DR | 0x00 | 数据寄存器（读写引脚电平） |
| GDIR | 0x04 | 方向寄存器（0=输入，1=输出） |
| PSR | 0x08 | 引脚状态寄存器（只读，实际引脚电平） |
| ICR1 | 0x0C | 中断配置寄存器 1（pin 0-15） |
| ICR2 | 0x10 | 中断配置寄存器 2（pin 16-31） |
| IMR | 0x14 | 中断屏蔽寄存器 |
| ISR | 0x18 | 中断状态寄存器 |
| EDGE_SEL | 0x1C | 边沿选择寄存器 |

### 输出模式编程步骤

1. 通过 IOMUXC `SW_MUX_CTL` 配置引脚为 GPIO ALT 模式
2. 设置 GDIR 对应位 = 1（输出方向）
3. 写 DR 寄存器控制输出电平

> HAL 的 `Port::output(pad)` 自动完成步骤 1-2，包括调用 `iomuxc::gpio::prepare()` 设置 ALT 模式。

### 中断配置

| ICR 值 | 触发方式 |
|--------|----------|
| 00 | 低电平触发 |
| 01 | 高电平触发 |
| 10 | 上升沿触发 |
| 11 | 下降沿触发 |

## 时钟架构要点（参考手册 Chapter 13-14）

- 主晶振：24 MHz
- RTC 晶振：32.768 kHz
- 内核时钟通过 PLL 倍频至 600 MHz
- **外设使用前必须使能对应的时钟门控**（CCM Clock Gating）
- GPIO 时钟门控：`hal::ccm::clock_gate::gpio::<N>().set(&mut ccm, ON)`

## 常用外设对照表

| 外设 | 数量 | HAL 模块 | 备注 |
|------|------|----------|------|
| GPIO | 5 端口 (GPIO1-5) | `hal::gpio` | GPIO1/2 为常规，GPIO5 为快速 GPIO |
| GPT (通用定时器) | 2 | `hal::gpt` | 32-bit，适合精确延时 |
| PIT (周期中断定时器) | 4 通道 | `hal::pit` | 32-bit 倒计数 |
| LPUART | 8 | `hal::lpuart` | 异步串口 |
| LPSPI | 4 | `hal::lpspi` | SPI 主/从 |
| LPI2C | 4 | `hal::lpi2c` | I2C |
| FlexPWM | 4 | `hal::flexpwm` | 电机控制级 PWM |
| USB OTG | 2 | `imxrt-usbd` | 集成 PHY |
| ADC | 2 (各 16 通道) | `hal::adc` | 12-bit |
| FlexCAN | 2 + 1 FD | — | CAN 2.0 + CAN-FD |

## 资料文件索引

| 文件 | 内容 |
|------|------|
| `快速入门指南 – MIMXRT1060-EVK [IMXRT1060QSG].pdf` | 开箱指南，板卡接口标注 |
| `SPF-31357_A33---MIMXRT1060_EVK -- SCH.pdf` | **原理图**（17 页，引脚连接最权威来源） |
| `IMXRT1060CEC---i.MX RT1060 跨界 MCU 数据表.pdf` | MCU 电气参数、封装、引脚分配 |
| `IMXRT1060RM---i.MX RT1060 处理器参考手册.pdf` | **完整参考手册**（寄存器定义、外设编程） |
| `AN12108---如何启用从 QSPI Flash 启动.pdf` | QSPI Flash 启动配置步骤 |
| `AN12183---如何启用 FLEXSPI NOR Flash 的调试功能.pdf` | XIP 调试、FCB 配置、镜像布局 |
| `i.MX RT系列跨界MCU产品手册[IMXRTPORTBR].pdf` | RT 系列产品概览 |
| `RT1060.bsdl` | JTAG 边界扫描描述文件 |
| `MIMXRT1060-EVK-DESIGN-FILE-A2.zip` | 评估套件设计文件（PCB 等） |

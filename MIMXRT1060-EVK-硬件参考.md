# MIMXRT1060-EVK 硬件参考

基于 `MIMXRT1060-EVK-资料/` 中的原理图、数据表（IMXRT1060CEC Rev.4）、参考手册（IMXRT1060RM Rev.3）及应用笔记整理。

---

## MCU 核心参数

| 项目 | 规格 |
|------|------|
| 型号 | MIMXRT1062DVL6A |
| 内核 | Arm Cortex-M7（单核，6 级超标量流水线，双发射） |
| 最高主频 | **600 MHz** |
| FPU | VFPv5，单精度 + 双精度 |
| L1 I-Cache | 32 KB |
| L1 D-Cache | 32 KB |
| MPU | 16 个保护区域 |
| CoreMark | ~3020 @ 600 MHz（5.03 CoreMark/MHz） |
| DMIPS | ~1284 @ 600 MHz（2.14 DMIPS/MHz） |
| 调试 | Arm CoreSight，TPIU 实时跟踪，5-pin JTAG + SWD |
| 封装 | 196-pin MAPBGA, 10×10 mm, 0.65 mm pitch |

### 运行模式与频率

| 模式 | VDD_SOC_IN | 内核频率 |
|------|-----------|---------|
| Overdrive Run | 1.25 - 1.3 V | 600 MHz |
| Normal Run | 1.15 - 1.3 V | 528 MHz |
| Low-speed Run | 1.15 - 1.3 V | 132 MHz |
| Low-power Run | 0.925 - 1.3 V | 24 MHz |

---

## 片上存储

| 存储 | 容量 | 说明 |
|------|------|------|
| FlexRAM | 512 KB | 以 32 KB 粒度灵活分配为 ITCM / DTCM / OCRAM，零等待 |
| 专用 OCRAM | 512 KB | 通用片上 RAM（OCRAM2） |
| **总 SRAM** | **1 MB** | |
| Boot ROM | 128 KB（96 KB 可用） | 含 ROM API |

---

## 外部存储接口

| 接口 | 说明 |
|------|------|
| **FlexSPI ×2** | 各双通道 Quad SPI；支持 Quad/Octal Flash、HyperFlash/RAM；SDR/DDR 最高 166 MHz；支持 XIP |
| **SEMC** | SDRAM（8/16-bit，最高 166 MHz）、并行 NOR Flash（支持 XIP）、SLC NAND、PSRAM、8080 显示接口 |
| **uSDHC ×2** | SD 3.0（SDR104，104 MB/s）、eMMC 4.5（HS200，200 MB/s）、支持 SDXC（最大 2 TB）；1/4/8-bit |

---

## 完整外设列表

### 通信接口

| 外设 | 数量 | 关键参数 |
|------|------|----------|
| **LPUART** | 8 | 7/8-bit 数据，最高 **20 Mbps**，DMA 支持 |
| **LPSPI** | 4 | 主/从模式，最高 30 MHz，DMA 支持 |
| **LPI2C** | 4 | Sm 100k / Fm 400k / Fm+ 1M / UFm 5M / Hs-mode 3.4M（从机） |
| **USB 2.0 OTG** | 2 | **集成高速 PHY**，480 Mbps，8+8 端点，OTG 主/从，USB BC 1.2 |
| **10/100 Ethernet** | 2 | IEEE 802.3 MAC，MII/RMII，**IEEE 1588** 精密时间协议硬件支持 |
| **FlexCAN** | 2 | CAN 2.0B，标准帧/扩展帧 |
| **FlexCAN FD** | 1 | CAN-FD，有效载荷最大 **64 字节**，最高 **8 Mbps** |
| **FlexIO** | 3 | 可编程 I/O 引擎，可模拟 UART/SPI/I2C/I2S/Camera/LCD/PWM 等 |

### 模拟外设

| 外设 | 数量 | 关键参数 |
|------|------|----------|
| **ADC** | 2 | **12-bit**（也支持 10/8-bit），共 20 通道 |
| | | 转换时钟 4-40 MHz，最快 **0.7 μs** 转换 |
| | | 校准后 ENOB ≥ 10.1，DNL 0.76 LSB typ，INL 2.78 LSB typ |
| | | 活跃功耗 350-750 μA，关断 1.4 μA |
| **ACMP** | 4 | 轨到轨模拟比较器 |
| | | 高速模式传播延迟 **25 ns** typ（40 ns max） |
| | | 低速模式 50 ns typ（90 ns max） |
| | | 可编程迟滞 1-64 mV，各内置 **6-bit DAC** |
| **TSC** | 1 | 4/5 线电阻式触摸屏控制器 |
| **TEMPMON** | 1 | 片上温度监测，可编程校准 |
| **ADC_ETC** | 1 | ADC 外部触发控制器，协调多源触发 |

### 定时器 / PWM

| 外设 | 数量 | 关键参数 |
|------|------|----------|
| **GPT** | 2 | **32-bit** 通用定时器，4 通道捕获/比较，自由运行/设置遗忘模式 |
| **PIT** | 4 通道 | **32-bit** 周期中断定时器，支持 DMA 触发 |
| **Quad Timer** | 4（各 4 通道） | **16-bit**，共 16 通道；上/下计数、级联、正交解码、可编程输入滤波 |
| **FlexPWM** | 4（各 4 子模块） | **16-bit**，最高 **150 MHz** 时钟，共最多 32 路 PWM 输出 |
| | | 电机控制级，故障输入，半桥驱动 |
| **QDC（正交编码器）** | 4 | 轴位置/转速测量，5 路输入（PHASEA/B、INDEX、TRIGGER、HOME） |

### 看门狗

| 外设 | 说明 |
|------|------|
| **WDOG1 / WDOG2** | 标准看门狗，双比较点，中断 + 外部复位 |
| **RTWDOG (WDOG3)** | 高可靠独立看门狗，支持窗口刷新模式 |
| **EWM** | 外部看门狗监视器，独立触发引脚 |

### 显示 / 摄像头 / 图形

| 外设 | 说明 |
|------|------|
| **eLCDIF** | 并行 RGB LCD 控制器，8/16/24-bit 数据宽度 |
| | 最高 **WXGA (1280×800)**，像素时钟 **75 MHz** |
| | 256 色 LUT，支持 8/16-bit 8080 MPU 接口 |
| **CSI** | CMOS 摄像头接口，8/10/16/24-bit 输入 |
| | RGB888/YUV444/CCIR656/Bayer，像素时钟 **80 MHz** |
| **PXP** | 2D 像素处理引擎，1 pixel/clock |
| | 色彩空间转换、Alpha 混合、Chroma Key、Porter-Duff 混合 |
| | 图像旋转（90°/180°/270°）、缩放、BitBlit、2D-DMA |

### 音频

| 外设 | 数量 | 说明 |
|------|------|------|
| **SAI** | 3 | I2S / AC97 / TDM / Codec/DSP，全双工，MCLK 最高 66.7 MHz，BCLK 最高 25 MHz |
| **SPDIF** | 1 | S/PDIF 数字音频收发，25 MHz |
| **MQS** | 1 | 中等质量声音，2 通道 PWM 音频输出（仅需 GPIO 引脚） |

### DMA 与互联

| 模块 | 说明 |
|------|------|
| **eDMA** | **32 通道**，DMA MUX 支持最多 **128 个请求源** |
| **XBAR（交叉开关）** | 3 个实例（XBARA、XBARB、XBAR3），外设间信号灵活路由 |
| **AOI** | 2 个实例，4 项求和积布尔函数生成器（每项 4 输入 A/B/C/D） |

### GPIO

| 项目 | 规格 |
|------|------|
| 标准 GPIO | GPIO1-5 |
| 高速 GPIO | GPIO6-9（与内核同频 **600 MHz**，紧耦合） |
| 总 GPIO 引脚 | **127**（124 紧耦合） |
| 驱动强度 | 7 级可编程（23-157 Ω @ 3.3V / 37-260 Ω @ 1.8V） |
| I/O 电压 | 1.8V / 3.3V（按供电组 NVCC_GPIO/SD0/SD1/EMC） |
| 转换速率 | 慢速/快速可选 |
| 上拉电阻 | 22k / 47k / 100k 上拉，100k 下拉，Keeper 电路 105-175 kΩ |
| 输入迟滞 | ≥ 250 mV（可使能） |
| LVDS | 1 对差分 I/O（CCM_CLK1_P/N），TIA/EIA 644-A 兼容 |

### 安全模块

| 模块 | 功能 |
|------|------|
| **HAB** | 安全启动认证（High Assurance Boot） |
| **DCP** | 加密协处理器：AES-128（ECB/CBC）、SHA-1、SHA-256、CRC-32 |
| **BEE** | 总线加密引擎：AES-128（ECB/CTR），QSPI Flash 实时解密 |
| **TRNG** | 真随机数生成器，512-bit 熵 |
| **SNVS** | 安全非易失存储 + 安全 RTC + 安全状态机 + ZMK |
| **SJC** | 安全 JTAG 控制器，3 级安全模式（eFuse 可选） |
| **CSU** | 中央安全单元，平台级安全策略 |
| **OCOTP** | OTP/eFuse 控制器，密钥与配置存储 |

### 其它

| 外设 | 说明 |
|------|------|
| **KPP** | 8×8 键盘矩阵接口，多键检测、长按检测、待机按键唤醒 |

---

## 时钟系统

### 外部时钟源

| 时钟 | 频率 | 类型 |
|------|------|------|
| 主晶振 XTALI | 24.0 MHz | 晶体或外部振荡器 |
| RTC 晶振 | 32.768 kHz | 晶体（或片内环振 ~40 kHz，±50%） |

### PLL

| PLL | 输出频率 | 用途 | 锁定时间 |
|-----|----------|------|----------|
| Arm PLL | 648-1296 MHz | 内核时钟 | < 2250 参考周期 |
| System PLL (PLL2) | 528 MHz | 系统总线 | < 11250 参考周期 |
| USB PLL (PLL3) | 480 MHz | USB | < 383 参考周期 |
| Audio/Video PLL | 650 MHz-1.3 GHz | 音视频 | < 11250 参考周期 |
| Ethernet PLL | 1 GHz | 以太网 | < 11250 参考周期 |

> **外设使用前必须使能对应的时钟门控**（CCM Clock Gating）
> GPIO 示例：`hal::ccm::clock_gate::gpio::<N>().set(&mut ccm, ON)`

---

## 电源管理

### 片内稳压器

| 模块 | 说明 |
|------|------|
| **DCDC** | Buck 转换器，3.3V 输入（2.8-3.6V）→ 0.8-1.575V 输出（25 mV 步进），效率 90%，最大 500 mA |
| | 过流保护 1A、过压保护 1.55V、低压保护 2.6V |
| **LDO_1P1** | 1.0-1.2V 输出，供 USB PHY / PLL |
| **LDO_2P5** | 2.25-2.75V 输出，供 USB PHY / eFuse / PLL，含弱稳压低功耗模式 |
| **LDO_USB** | 3.0V 输出，由 VBUS 供电（4.4-5.5V），限流保护 |
| **LDO_SNVS** | SNVS 域数字稳压器，支持 Power Gate 和 Analog 模式 |
| **GPC** | 通用电源控制器，硬件功耗管理与门控 |

### 低功耗电流

| 模式 | 总功耗 | 说明 |
|------|--------|------|
| System IDLE | **38.72 mW** | CPU WFI，1 MB RAM 保持 |
| Low Power IDLE | **4.84 mW** | |
| SUSPEND | **0.789 mW** | DSM，64 KB RAM 保持 |
| SNVS | **0.066 mW** | 仅 RTC 运行 |

### 最大供电电流

| 供电轨 | 最大电流 |
|---------|---------|
| DCDC_IN（CoreMark 运行，95°C） | 110 mA |
| VDD_HIGH_IN | 50 mA |
| VDD_SNVS_IN | 250 μA |
| USB VBUS（每路） | 25 mA（双路共 50 mA） |
| VDDA_ADC_3P3 | 40 mA（含触摸屏负载） |

---

## 封装与温度

| 封装 | 尺寸 | Pitch | 引脚 |
|------|------|-------|------|
| MAPBGA | 10×10 mm | 0.65 mm | 196 |
| MAPBGA | 12×12 mm | 0.8 mm | 196 |

| 温度等级 | 结温范围 |
|----------|---------|
| 商业级 (D) | 0 ~ +95°C |
| 工业级 (C) | -40 ~ +105°C |
| 扩展工业级 (X) | -40 ~ +125°C |

| 热阻（10×10 mm, 2S2P 板） | 值 |
|---------------------------|------|
| Junction-to-Ambient (RJA) | 40.8°C/W |
| Junction-to-Case (RJC) | 16.8°C/W |

| ESD 等级 | 额定值 |
|----------|--------|
| HBM | 1000 V |
| CDM | 500 V |

---

## 型号变体

| 型号 | 差异 | 封装 |
|------|------|------|
| MIMXRT1062DVL6A/B | 全功能（含 LCD/CSI/PXP） | 10×10 mm |
| MIMXRT1062DVJ6A/B | 全功能（含 LCD/CSI/PXP） | 12×12 mm |
| MIMXRT1061DVL6A/B | **无** LCD/CSI/PXP | 10×10 mm |
| MIMXRT106A... | 语音（Alexa AFE + VoiceSpot） | 10×10 mm |
| MIMXRT106F... | 人脸/情绪识别交钥匙方案 | 10×10 mm |
| MIMXRT106S... | 本地语音（音素识别） | 10×10 mm |

> 后缀 A = 硅版本 A0，B = 硅版本 A1；位置 6 的 "6" = 600 MHz。

---

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

### 板载外设

| 功能 | 器件/型号 | 接口 | 备注 |
|------|-----------|------|------|
| Ethernet PHY | KSZ8081 | RMII | 100Base-T |
| Audio Codec | WM8960 | I2S (SAI) | |
| 运动传感器 | FXOS8700CQ | I2C | 加速度计 + 磁力计 |
| Camera | OV7725 | CSI | |
| LCD | 4.3" TFT 480×272 | 并行 RGB (eLCDIF) | |
| USB OTG | J9 | USB OTG1 | |
| Debug USB | J41 (OpenSDA) | — | 串口 + DAP-Link 调试 |
| 用户按钮 | SW8 (USER_BUTTON) | GPIO | |
| 复位按钮 | SW3 (POR_BUTTON) | — | |

### 启动模式拨码开关 SW7

| SW7-1 | SW7-2 | SW7-3 | SW7-4 | 启动设备 |
|-------|-------|-------|-------|----------|
| OFF | ON | ON | OFF | HyperFlash |
| OFF | OFF | ON | OFF | **QSPI NOR Flash** |
| ON | OFF | ON | OFF | SD Card |

- BOOT_MODE[1:0] = 10b → Internal Boot（正常启动模式）
- BOOT_MODE[1:0] = 01b → Serial Downloader（下载模式）

### JTAG/SWD 调试接口 J21（原理图 Sheet 12）

J21 为标准 ARM 20-pin JTAG 接口（2×10，0.05" 间距）：

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

---

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

---

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

---

## 外设数量汇总

| 类别 | 外设 | 数量 |
|------|------|------|
| **处理器** | Arm Cortex-M7 | 1 |
| **缓存** | L1 I-Cache / D-Cache | 32 KB + 32 KB |
| **存储** | Boot ROM / FlexRAM / OCRAM | 128 KB / 512 KB / 512 KB |
| **DMA** | eDMA 通道 / MUX 源 | 32 / 128 |
| **串口** | LPUART | 8 |
| **SPI** | LPSPI | 4 |
| **I2C** | LPI2C | 4 |
| **CAN** | FlexCAN + FlexCAN FD | 2 + 1 |
| **可编程 I/O** | FlexIO | 3 |
| **USB** | USB 2.0 OTG（集成 PHY） | 2 |
| **以太网** | 10/100 MAC (IEEE 1588) | 2 |
| **SD/eMMC** | uSDHC | 2 |
| **外部存储** | FlexSPI / SEMC | 2 / 1 |
| **ADC** | 12-bit ADC（共 20 通道） | 2 |
| **比较器** | ACMP（含 6-bit DAC） | 4 |
| **触摸屏** | TSC | 1 |
| **温度监测** | TEMPMON | 1 |
| **通用定时器** | GPT (32-bit) | 2 |
| **周期定时器** | PIT (32-bit) | 4 通道 |
| **四路定时器** | Quad Timer (16-bit, 4ch/实例) | 4（共 16 通道） |
| **PWM** | FlexPWM (16-bit, 150 MHz) | 4（共最多 32 路） |
| **编码器** | QDC | 4 |
| **看门狗** | WDOG / RTWDOG / EWM | 2 + 1 + 1 |
| **音频** | SAI / SPDIF / MQS | 3 / 1 / 1 |
| **显示** | eLCDIF | 1 |
| **摄像头** | CSI | 1 |
| **图形** | PXP | 1 |
| **键盘** | KPP (8×8) | 1 |
| **互联** | XBAR / AOI / ADC_ETC | 3 / 2 / 1 |
| **GPIO** | 标准 + 高速 | 5 + 4（共 127 引脚） |
| **安全** | HAB / DCP / BEE / TRNG / SNVS / SJC / CSU / OCOTP | 8 个模块 |
| **电源** | DCDC / LDO×4 / GPC | 6 个模块 |

---

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

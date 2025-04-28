#![no_std]

//! An embedded async driver for the ICM-42605 6-Axis IMU.

device_driver::create_device! {
    device_name: Icm42605,
    dsl: {
        config {
            type RegisterAddressType = u16;
            type DefaultByteOrder = BE;
            type DefmtFeature = "defmt";
        }

        block UserBank0 {
            const ADDRESS_OFFSET = 0x0000;

            register DeviceConfig {
                const ADDRESS = 0x11;
                const SIZE_BITS = 8;

                spi_mode: bool = 4,
                soft_reset_config: bool = 0,
            },

            register DriveConfig {
                const ADDRESS = 0x13;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x05;

                i2c_slew_rate: uint = 3..6,
                spi_slew_rate: uint = 0..3,
            },

            register IntConfig {
                const ADDRESS = 0x14;
                const SIZE_BITS = 8;

                int2_mode: bool = 5,
                int2_drive_circuit: bool = 4,
                int2_polarity: bool = 3,
                int1_mode: bool = 2,
                int1_drive_circuit: bool = 1,
                int1_polarity: bool = 0,
            },

            register FifoConfig {
                const ADDRESS = 0x16;
                const SIZE_BITS = 8;

                fifo_mode: uint as enum FifoMode {
                    Bypass,
                    Stream,
                    StopOnFull,
                    Reserved,
                } = 6..8,
            },

            register TempData {
                type Access = RO;
                const ADDRESS = 0x1d;
                const SIZE_BITS = 16;
                const RESET_VALUE = 0x8000;

                value: int = 0..16,
            },

            ref AccelDataX = register TempData {
                const ADDRESS = 0x1f;
            },

            ref AccelDataY = register TempData {
                const ADDRESS = 0x21;
            },

            ref AccelDataZ = register TempData {
                const ADDRESS = 0x23;
            },

            ref GyroDataX = register TempData {
                const ADDRESS = 0x25;
            },

            ref GyroDataY = register TempData {
                const ADDRESS = 0x27;
            },

            ref GyroDataZ = register TempData {
                const ADDRESS = 0x29;
            },

            register TimestampFsync {
                type Access = RO;
                const ADDRESS = 0x2b;
                const SIZE_BITS = 16;

                value: int = 0..16,
            },

            register IntStatus {
                type Access = RO;
                const ADDRESS = 0x2d;
                const SIZE_BITS = 8;

                ui_fsync_int: bool = 6,
                pll_rdy_int: bool = 5,
                reset_done_int: bool = 4,
                data_rdy_int: bool = 3,
                fifo_ths_int: bool = 2,
                fifo_full_int: bool = 1,
                agc_rdy_int: bool = 0,
            },

            register FifoCount {
                type Access = RO;
                const ADDRESS = 0x2e;
                const SIZE_BITS = 16;

                value: int = 0..16,
            },

            register FifoData {
                type Access = RO;
                const ADDRESS = 0x30;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0xff;

                value: int = 0..8,
            },

            block ApexData {
                register StepCount {
                    type Access = RO;
                    type ByteOrder = LE;
                    const ADDRESS = 0x31;
                    const SIZE_BITS = 16;

                    value: uint = 0..16,
                },

                register StepCadence {
                    type Access = RO;
                    const ADDRESS = 0x33;
                    const SIZE_BITS = 8;

                    value: uint = 0..8,
                },

                register ApexStatus {
                    type Access = RO;
                    const ADDRESS = 0x34;
                    const SIZE_BITS = 8;
                    const RESET_VALUE = 0x04;

                    dmp_idle: bool = 2,
                    activity_class: uint as enum ActivityClass {
                        Unknown,
                        Walk,
                        Run,
                        Reserved,
                    } = 0..2,
                },

                register TapStatus {
                    type Access = RO;
                    const ADDRESS = 0x35;
                    const SIZE_BITS = 8;

                    tap_num: uint as enum TapNum {
                        NoTap,
                        Single,
                        Double,
                        Reserved,
                    } = 3..5,
                    tap_axis: uint as enum Axis {
                        X,
                        Y,
                        Z,
                        Reserved,
                    } = 1..3,
                    tap_dir: uint as enum Polarity {
                        Positive,
                        Negative,
                    } = 0..1,
                },

                register DoubleTapTiming {
                    type Access = RO;
                    const ADDRESS = 0x36;
                    const SIZE_BITS = 8;

                    value: uint = 0..6,
                },
            },

            register IntStatus2 {
                type Access = RO;
                const ADDRESS = 0x37;
                const SIZE_BITS = 8;

                smd_int: bool = 3,
                wom_z_int: bool = 2,
                wom_y_int: bool = 1,
                wom_x_int: bool = 0,
            },

            register IntStatus3 {
                type Access = RO;
                const ADDRESS =  0x38;
                const SIZE_BITS = 8;

                step_det_int: bool = 5,
                step_cnt_ovf_int: bool = 4,
                tilt_det_int: bool = 3,
                wake_int: bool = 2,
                sleep_int: bool = 1,
                tap_det_int: bool = 0,
            },

            register SignalPathReset {
                type Access = WO;
                const ADDRESS = 0x4b;
                const SIZE_BITS = 8;

                dmp_init_en: bool = 6,
                dmp_mem_reset_en: bool = 5,
                abort_and_reset: bool = 3,
                tmst_strobe: bool = 2,
                fifo_flush: bool = 1,
            },

            register IntfConfig0 {
                const ADDRESS = 0x4c;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x30;

                fifo_hold_last_data_en: bool = 7,
                fifo_count_rec: uint as enum FifoCountRec {
                    Bytes,
                    Records,
                } = 6..7,
                fifo_count_endian: uint as enum Endian {
                    LittleEndian,
                    BigEndian,
                } = 5..6,
                sensor_data_endian: uint as Endian = 4..5,
                ui_sifs_cfg: uint as enum UiSifsCfg {
                    Reserved = catch_all,
                    DisableSpi = 2,
                    DisableI2c = 3,
                } = 0..2,
            },

            register IntfConfig1 {
                const ADDRESS = 0x4d;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x91;

                accel_lp_clk_sel: uint as enum AccelLpClkSel {
                    WakeUp,
                    Rc,
                } = 3..4,
                clksel: uint as enum ClkSel {
                    InternalRc,
                    PllOrRc,
                    Reserved,
                    Disable,
                } = 0..2,
            },

            register PwrMgmt {
                const ADDRESS = 0x4e;
                const SIZE_BITS = 8;

                temp_dis: bool = 5,
                idle: bool = 4,
                gyro_mode: uint as enum GyroMode {
                    Off,
                    Standby,
                    Reserved,
                    LowNoise,
                } = 2..4,
                accel_mode: uint as enum AccelMode {
                    Off,
                    Reserved,
                    LowPower,
                    LowNoise,
                } = 0..2,
            },

            register GyroConfig0 {
                const ADDRESS = 0x4f;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x06;

                gyro_fs_sel: uint as enum GyroFullScale {
                    DegreesPerSec2000,
                    DegreesPerSec1000,
                    DegreesPerSec500,
                    DegreesPerSec250,
                    DegreesPerSec125,
                    DegreesPerSec62x5,
                    DegreesPerSec31x25,
                    DegreesPerSec15x625,
                } = 5..8,
                gyro_odr: uint as enum DataRate {
                    Reserved = catch_all,
                    Hz8000 = 3,
                    Hz4000 = 4,
                    Hz2000 = 5,
                    Hz1000 = 6,
                    Hz200 = 7,
                    Hz100 = 8,
                    Hz50 = 9,
                    Hz25 = 10,
                    Hz12x5 = 11,
                    Hz6x25 = 12,
                    Hz3x125 = 13,
                    Hz1x15625 = 14,
                    Hz500 = 15,
                } = 0..4,
            },

            register AccelConfig0 {
                const ADDRESS = 0x50;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x06;

                gyro_fs_sel: uint as enum AccelFullScale {
                    Max16G,
                    Max8G,
                    Max4G,
                    Max2G,
                    Reserved = catch_all,
                } = 5..8,
                gyro_odr: uint as DataRate = 0..4,
            },

            register GyroConfig1 {
                const ADDRESS = 0x51;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x16;

                temp_filt_bw: uint as enum LowPassFilterLatency {
                    Latency125ms,
                    Latency1ms,
                    Latency2ms,
                    Latency4ms,
                    Latency8ms,
                    Latency16ms,
                    Latency32ms,
                    Reserved,
                } = 5..8,
                gyro_ui_filt_ord: uint as enum UiFilterOrder {
                    FirstOrder,
                    SecondOrder,
                    ThirdOrder,
                    Reserved,
                } = 2..4,
                gyro_dec2_m2_ord: uint as enum Dec2M2FilterOrder {
                    Reserved = catch_all,
                    ThirdOrder = 2,
                } = 0..2,
            },

            register GyroAccelConfig {
                const ADDRESS = 0x52;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x11;

                accel_ui_filt_bw: uint = 4..8,
                gyro_ui_filt_bw: uint = 0..4,
            },

            register AccelConfig1 {
                const ADDRESS = 0x53;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x0d;

                accel_ui_filt_ord: uint as UiFilterOrder = 3..5,
                accel_dec2_m2_ord: uint as Dec2M2FilterOrder = 1..3,
            },

            register TmstConfig {
                const ADDRESS = 0x54;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x23;

                tmst_to_regs_en: bool = 4,
                tmst_res: bool = 3,
                tmst_delta_en: bool = 2,
                tmst_fsync_en: bool = 1,
                tmst_en: bool = 0,
            },

            register ApexConfig {
                const ADDRESS = 0x56;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x82;

                dmp_power_save: bool = 7,
                tap_enable: bool = 6,
                ped_enable: bool = 5,
                tilt_enable: bool = 4,
                r2w_en: bool = 3,
                dmp_odr: uint as enum DmpDataRate {
                    Hz25,
                    Reserved = catch_all,
                    Hz50 = 2
                } = 0..2,
            },

            register SmdConfig {
                const ADDRESS = 0x57;
                const SIZE_BITS = 8;

                wom_int_mode: bool = 3,
                wom_mode: bool = 2,
                smd_mode: uint as enum SmdMode {
                    Disabled,
                    WakeOnMotion,
                    Short,
                    Long,
                } = 0..2,
            },

            register FifoConfig1 {
                const ADDRESS = 0x5f;
                const SIZE_BITS = 8;

                fifo_resume_partial_rd: bool = 6,
                fifo_wm_gt_th: bool = 5,
                fifo_tmst_fsync_en: bool = 3,
                fifo_temp_en: bool = 2,
                fifo_gyro_en: bool = 1,
                fifo_accel_en: bool = 0,
            },

            register FifoWatermark {
                type ByteOrder = LE;
                const ADDRESS = 0x60;
                const SIZE_BITS = 16;

                value: uint = 0..12,
            },

            register FsyncConfig {
                const ADDRESS = 0x62;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x10;

                fsync_ui_sel: uint as enum FsyncFlag {
                    Disabled,
                    Temp,
                    GyroX,
                    GyroY,
                    GyroZ,
                    AccelX,
                    AccelY,
                    AccelZ,
                } = 4..7,
                fsync_ui_flag_clear_sel: bool = 1,
                fsync_polarity: bool = 0,
            },

            register IntConfig0 {
                const ADDRESS = 0x63;
                const SIZE_BITS = 8;

                ui_drdy_int_clear: uint as enum IntClearOption {
                    StatusBitRead,
                    Reserved,
                    SensorRegisterRead,
                    Both,
                } = 4..6,
                fifo_ths_int_clear: uint as IntClearOption = 2..4,
                fifo_full_int_clear: uint as IntClearOption = 0..2,
            },

            register IntConfig1 {
                const ADDRESS = 0x64;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x10;

                int_tpulse_duration: bool = 6,
                int_tdeassert_disable: bool = 5,
                int_async_reset: bool = 4,
            },

            register IntSource0 {
                const ADDRESS = 0x65;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x10;

                ui_fsync_int1_en: bool = 6,
                pll_rdy_int1_en: bool = 5,
                reset_down_int1_en: bool = 4,
                ui_drdy_int1_en: bool = 3,
                fifo_ths_int1_en: bool = 2,
                fifo_full_int1_en: bool = 1,
                ui_agc_rdy_int1_en: bool = 0,
            },

            register IntSource1 {
                const ADDRESS = 0x66;
                const SIZE_BITS = 8;

                i3c_protocol_error_int1_en: bool = 6,
                smd_int1_en: bool = 3,
                wom_z_int1_en: bool = 2,
                wom_y_int1_en: bool = 1,
                wom_x_int1_en: bool = 0,
            },

            register IntSource3 {
                const ADDRESS = 0x68;
                const SIZE_BITS = 8;

                ui_fsync_int2_en: bool = 6,
                pll_rdy_int2_en: bool = 5,
                reset_down_int2_en: bool = 4,
                ui_drdy_int2_en: bool = 3,
                fifo_ths_int2_en: bool = 2,
                fifo_full_int2_en: bool = 1,
                ui_agc_rdy_int2_en: bool = 0,
            },

            register IntSource4 {
                const ADDRESS = 0x69;
                const SIZE_BITS = 8;

                i3c_protocol_error_int2_en: bool = 6,
                smd_int2_en: bool = 3,
                wom_z_int2_en: bool = 2,
                wom_y_int2_en: bool = 1,
                wom_x_int2_en: bool = 0,
            },

            register FifoLostPkt {
                type Access = RO;
                type ByteOrder = LE;
                const ADDRESS = 0x6c;
                const SIZE_BITS = 16;

                value: uint = 0..16,
            },

            register SelfTestConfig {
                const ADDRESS = 0x70;
                const SIZE_BITS = 8;

                accel_st_power: bool = 6,
                en_az_st: bool = 5,
                en_ay_st: bool = 4,
                en_ax_st: bool = 3,
                en_gz_st: bool = 2,
                en_gy_st: bool = 1,
                en_gx_st: bool = 0,
            },

            register WhoAmI {
                type Access = RO;
                const ADDRESS = 0x75;
                const SIZE_BITS = 8;

                whoami: uint = 0..8,
            },
        },

        block UserBank1 {
            const ADDRESS_OFFSET = 0x0100;

            register SensorConfig {
                const ADDRESS = 0x03;
                const SIZE_BITS = 8;

                zg_disable: bool = 5,
                yg_disable: bool = 4,
                xg_disable: bool = 3,
                za_disable: bool = 2,
                ya_disable: bool = 1,
                xa_disable: bool = 0,
            },

            register GyroConfigStatic2 {
                const ADDRESS = 0x0b;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0xa8;

                gyro_aaf_dis: bool = 1,
                gyro_nf_dis: bool = 0,
            },

            register GyroAntiAliasFilterConfig {
                type ByteOrder = LE;
                const ADDRESS = 0x0c;
                const SIZE_BITS = 24;
                const RESET_VALUE = 0x3f;

                gyro_aaf_delt: uint = 0..6,
                gyro_aaf_deltsqr: uint = 8..20,
                gyro_aaf_bitshift: uint = 20..24,
            },

            register GyroXNfCoswz {
                const ADDRESS = 0x0f;
                const SIZE_BITS = 8;

                value: uint = 0..8,
            },

            ref GyroYNfCoswz = register GyroXNfCoswz {
                const ADDRESS = 0x10;
            },

            ref GyroZNfCoswz = register GyroXNfCoswz {
                const ADDRESS = 0x11;
            },

            register GyroConfigStatic9 {
                const ADDRESS = 0x12;
                const SIZE_BITS = 8;

                gyro_z_nf_coswz_sel: bool = 5,
                gyro_y_nf_coswz_sel: bool = 4,
                gyro_x_nf_coswz_sel: bool = 3,
                gyro_z_nf_coswz: bool = 2,
                gyro_y_nf_coswz: bool = 1,
                gyro_x_nf_coswz: bool = 0,
            },

            register GyroConfigStatic10 {
                const ADDRESS = 0x13;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x11;

                gyro_nf_bw_sel: uint = 4..7,
            },

            ref XgStData  = register GyroXNfCoswz {
                const ADDRESS = 0x5f;
            },

            ref YgStData  = register GyroXNfCoswz {
                const ADDRESS = 0x60;
            },

            ref ZgStData  = register GyroXNfCoswz {
                const ADDRESS = 0x61;
            },

            register TmstVal {
                type ByteOrder = LE;
                const ADDRESS = 0x62;
                const SIZE_BITS = 24;

                value: uint = 0..20,
            },

            register IntfConfig4 {
                const ADDRESS = 0x7a;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x03;

                i3c_bus_mode: bool = 6,
                spi_ap_4wire: bool = 1,
            },

            register IntfConfig5 {
                const ADDRESS = 0x7b;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x20;

                pin9_function: uint as enum Pin9Function {
                    Int2,
                    FSync,
                    Reserved = catch_all,
                } = 1..3,
            },

            register IntfConfig6 {
                const ADDRESS = 0x7c;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x5f;

                asynctime0_dis: bool = 7,
                i3c_en: bool = 4,
                i3c_ibi_byte_en: bool = 3,
                i3c_ibi_en: bool = 2,
                i3c_ddr_en: bool = 1,
                i3c_sdr_en: bool = 0,
            },
        },

        block UserBank2 {
            const ADDRESS_OFFSET = 0x0200;

            register AccelAntiAliasFilterConfig {
                type ByteOrder = LE;
                const ADDRESS = 0x0c;
                const SIZE_BITS = 24;
                const RESET_VALUE = 0x3f;

                accel_aaf_dis: bool = 0,
                accel_aaf_delt: uint = 1..7,
                accel_aaf_deltsqr: uint = 8..20,
                accel_aaf_bitshift: uint = 20..24,
            },

            ref XaStData  = register GyroXNfCoswz {
                const ADDRESS = 0x3b;
            },

            ref YaStData  = register GyroXNfCoswz {
                const ADDRESS = 0x3c;
            },

            ref ZaStData  = register GyroXNfCoswz {
                const ADDRESS = 0x3d;
            },
        },

        block UserBank4 {
            const ADDRESS_OFFSET = 0x0400;

            register ApexConfig1 {
                const ADDRESS = 0x40;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0xa2;

                low_energy_amp_th_sel: uint = 4..8,
                dmp_power_save_time_sel: uint = 0..4,
            },

            register ApexConfig2 {
                const ADDRESS = 0x41;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x85;

                ped_amp_th_sel: uint = 4..8,
                ped_step_cnt_th_sel: uint =0..4,
            },

            register ApexConfig3 {
                const ADDRESS = 0x42;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x51;

                ped_step_det_th_sel: uint = 5..8,
                ped_sb_timer_th_sel: uint = 2..5,
                ped_hi_en_th_sel: uint = 0..2,
            },

            register ApexConfig4 {
                const ADDRESS = 0x43;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0xa4;

                tilt_wait_time_sel: uint = 6..8,
                sleep_time_out: uint = 3..6,
            },

            register ApexConfig5 {
                const ADDRESS = 0x44;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x8c;

                mounting_matrix: uint = 0..3,
            },

            register ApexConfig6 {
                const ADDRESS = 0x45;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x5c;

                sleep_gesture_delay: uint = 0..3,
            },

            register ApexConfig7 {
                const ADDRESS = 0x47;
                const SIZE_BITS = 8;
                const RESET_VALUE = 0x5b;

                tap_tmax: uint = 5..7,
                tap_tavg: uint = 3..5,
                tap_tmin: uint = 0..3,
            },

            register ApexConfig8 {
                const ADDRESS = 0x48;
                const SIZE_BITS = 8;

                sensitivity_mode: bool = 0,
            },

            register AccelWomXThr {
                const ADDRESS = 0x4a;
                const SIZE_BITS = 8;

                value: uint = 0..8,
            },

            ref AccelWomYThr = register AccelWomXThr {
                const ADDRESS = 0x4b;
            },

            ref AccelWomZThr = register AccelWomXThr {
                const ADDRESS = 0x4c;
            },

            register IntSource6 {
                const ADDRESS = 0x4d;
                const SIZE_BITS = 8;

                step_det_int1_en: bool = 5,
                step_cnt_ofl_int1_en: bool = 4,
                tilt_det_int1_en: bool = 3,
                wake_det_int1_en: bool = 2,
                sleep_det_int1_en: bool = 1,
                tap_det_int1_en: bool = 0,
            },

            register IntSource7 {
                const ADDRESS = 0x4e;
                const SIZE_BITS = 8;

                step_det_int2_en: bool = 5,
                step_cnt_ofl_int2_en: bool = 4,
                tilt_det_int2_en: bool = 3,
                wake_det_int2_en: bool = 2,
                sleep_det_int2_en: bool = 1,
                tap_det_int2_en: bool = 0,
            },

            register IntSource8 {
                const ADDRESS = 0x4f;
                const SIZE_BITS = 8;

                fsync_ibi_en: bool = 6,
                pll_rdy_ibi_en: bool = 5,
                reset_down_ibi_en: bool = 4,
                ui_drdy_ibi_en: bool = 3,
                fifo_ths_ibi_en: bool = 2,
                fifo_full_ibi_en: bool = 1,
                ui_agc_rdy_ibi_en: bool = 0,
            },

            register IntSource9 {
                const ADDRESS = 0x50;
                const SIZE_BITS = 8;

                i3c_protocol_error_ibi_en: bool = 7,
                smd_ibi_en: bool = 4,
                wom_z_ibi_en: bool = 3,
                wom_y_ibi_en: bool = 2,
                wom_x_ibi_en: bool = 1,
            },

            register IntSource10 {
                const ADDRESS = 0x51;
                const SIZE_BITS = 8;

                step_det_ibi_en: bool = 5,
                step_cnt_ofl_ibi_en: bool = 4,
                tilt_det_ibi_en: bool = 3,
                wake_det_ibi_en: bool = 2,
                sleep_det_ibi_en: bool = 1,
                tap_det_ibi_en: bool = 0,
            },

            register OffsetUser0 {
                const ADDRESS = 0x77;
                const SIZE_BITS = 8;

                gyro_x_offuser_lo: uint = 0..8,
            },

            register OffsetUser1 {
                const ADDRESS = 0x78;
                const SIZE_BITS = 8;

                gyro_y_offuser_hi: uint = 4..8,
                gyro_x_offuser_hi: uint = 0..4,
            },

            register OffsetUser2 {
                const ADDRESS = 0x79;
                const SIZE_BITS = 8;

                gyro_y_offuser_lo: uint = 0..8,
            },

            register OffsetUser3 {
                const ADDRESS = 0x7a;
                const SIZE_BITS = 8;

                gyro_z_offuser_lo: uint = 0..8,
            },

            register OffsetUser4 {
                const ADDRESS = 0x7b;
                const SIZE_BITS = 8;

                accel_x_offuser_hi: uint = 4..8,
                gyro_z_offuser_hi: uint = 0..4,
            },

            register OffsetUser5 {
                const ADDRESS = 0x7c;
                const SIZE_BITS = 8;

                accel_x_offuser_lo: uint = 0..8,
            },

            register OffsetUser6 {
                const ADDRESS = 0x7d;
                const SIZE_BITS = 8;

                accel_y_offuser_lo: uint = 0..8,
            },

            register OffsetUser7 {
                const ADDRESS = 0x7e;
                const SIZE_BITS = 8;

                accel_z_offuser_hi: uint = 4..8,
                accel_y_offuser_hi: uint = 0..4,
            },

            register OffsetUser8 {
                const ADDRESS = 0x7f;
                const SIZE_BITS = 8;

                accel_z_offuser_lo: uint = 0..8,
            },
        },
    }
}

const REG_BANK_SEL: u8 = 0x76;

pub struct Icm42605Interface<I2C> {
    i2c: I2C,
    addr: u8,
    bank: u8,
}

impl<I2C> Icm42605Interface<I2C> {
    pub fn new(i2c: I2C, ad0: bool) -> Self {
        Self {
            i2c,
            addr: 0x68 | u8::from(ad0),
            bank: 0xff,
        }
    }

    fn set_bank(&mut self, bank: u8) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        if bank != self.bank {
            self.i2c.write(self.addr, &[REG_BANK_SEL, bank])?;
            self.bank = bank;
        }
        Ok(())
    }

    async fn set_bank_async(&mut self, bank: u8) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal_async::i2c::I2c,
    {
        if bank != self.bank {
            self.i2c.write(self.addr, &[REG_BANK_SEL, bank]).await?;
            self.bank = bank;
        }
        Ok(())
    }
}

impl<I2C: embedded_hal::i2c::I2c> device_driver::RegisterInterface for Icm42605Interface<I2C> {
    type Error = I2C::Error;

    type AddressType = u16;

    fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        const MAX_LEN: usize = 3;
        assert!(data.len() < MAX_LEN);

        let [reg, bank] = address.to_le_bytes();
        self.set_bank(bank)?;

        let mut buf = heapless::Vec::<u8, MAX_LEN>::new();
        let _ = buf.push(reg);
        let _ = buf.extend_from_slice(data);
        self.i2c.write(self.addr, &buf)
    }

    fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        let [reg, bank] = address.to_le_bytes();
        self.set_bank(bank)?;
        self.i2c.write_read(self.addr, &[reg], data)
    }
}

impl<I2C: embedded_hal_async::i2c::I2c> device_driver::AsyncRegisterInterface
    for Icm42605Interface<I2C>
{
    type Error = I2C::Error;

    type AddressType = u16;

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        const MAX_LEN: usize = 3;
        assert!(data.len() < MAX_LEN);

        let [reg, bank] = address.to_le_bytes();
        self.set_bank_async(bank).await?;

        let mut buf = heapless::Vec::<u8, MAX_LEN>::new();
        let _ = buf.push(reg);
        let _ = buf.extend_from_slice(data);
        self.i2c.write(self.addr, &buf).await
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        let [reg, bank] = address.to_le_bytes();
        self.set_bank_async(bank).await?;
        self.i2c.write_read(self.addr, &[reg], data).await
    }
}

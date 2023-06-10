#![allow(dead_code)]

#[derive(Clone, Copy)]
pub enum Framesize {
    Normal,
    Short,
    Medium,
}

pub const FRAME_SIZE_NORMAL: usize = 64800;
pub const FRAME_SIZE_SHORT: usize = 16200;
pub const FRAME_SIZE_MEDIUM: usize = 32400;

impl Framesize {
    pub fn frame_size(&self) -> usize {
        match self {
            Framesize::Normal => FRAME_SIZE_NORMAL,
            Framesize::Short => FRAME_SIZE_SHORT,
            Framesize::Medium => FRAME_SIZE_MEDIUM,
        }
    }
}

#[derive(Clone, Copy)]
pub enum CodeRate {
    C1_4,
    C1_3,
    C2_5,
    C1_2,
    C3_5,
    C2_3,
    C3_4,
    C4_5,
    C5_6,
    C8_9,
    C9_10,
    C13_45,
    C9_20,
    C90_180,
    C96_180,
    C11_20,
    C100_180,
    C104_180,
    C26_45,
    C18_30,
    C28_45,
    C23_36,
    C116_180,
    C20_30,
    C124_180,
    C25_36,
    C128_180,
    C13_18,
    C132_180,
    C22_30,
    C135_180,
    C140_180,
    C7_9,
    C154_180,
    C11_45,
    C4_15,
    C14_45,
    C7_15,
    C8_15,
    C32_45,
    C2_9Vlsnr,
    C1_5Medium,
    C11_45Medium,
    C1_3Medium,
    C1_5VlsnrSf2,
    C11_45VlsnrSf2,
    C1_5Vlsnr,
    C4_15Vlsnr,
    C1_3Vlsnr,
    COther,
}

#[derive(Clone, Copy)]
pub enum Constellation {
    ModQpsk,
    Mod8psk,
    Mod8apsk,
    Mod16apsk,
    Mod8_8apsk,
    Mod32apsk,
    Mod4_12_16apsk,
    Mod4_8_4_16apsk,
    ModBpsk,
    ModBpskSf2,
    ModOther,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum RolloffFactor {
    Ro0_35,
    Ro0_25,
    Ro0_20,
    RoReserved,
    Ro0_15,
    Ro0_10,
    Ro0_05,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Modcod {
    McDummy,
    McDummyS,
    // DVB-S2:
    McQpsk1_4,
    McQpsk1_4S,
    McQpsk1_3,
    McQpsk1_3S,
    McQpsk2_5,
    McQpsk2_5S,
    McQpsk1_2,
    McQpsk1_2S,
    McQpsk3_5,
    McQpsk3_5S,
    McQpsk2_3,
    McQpsk2_3S,
    McQpsk3_4,
    McQpsk3_4S,
    McQpsk4_5,
    McQpsk4_5S,
    McQpsk5_6,
    McQpsk5_6S,
    McQpsk8_9,
    McQpsk8_9S,
    McQpsk9_10,
    McQpsk9_10S,
    Mc8psk3_5,
    Mc8psk3_5S,
    Mc8psk2_3,
    Mc8psk2_3S,
    Mc8psk3_4,
    Mc8psk3_4S,
    Mc8psk5_6,
    Mc8psk5_6S,
    Mc8psk8_9,
    Mc8psk8_9S,
    Mc8psk9_10,
    Mc8psk9_10S,
    Mc16apsk2_3,
    Mc16apsk2_3S,
    Mc16apsk3_4,
    Mc16apsk3_4S,
    Mc16apsk4_5,
    Mc16apsk4_5S,
    Mc16apsk5_6,
    Mc16apsk5_6S,
    Mc16apsk8_9,
    Mc16apsk8_9S,
    Mc16apsk9_10,
    Mc16apsk9_10S,
    Mc32apsk3_4,
    Mc32apsk3_4S,
    Mc32apsk4_5,
    Mc32apsk4_5S,
    Mc32apsk5_6,
    Mc32apsk5_6S,
    Mc32apsk8_9,
    Mc32apsk8_9S,
    Mc32apsk9_10,
    Mc32apsk9_10S,
    // DVB-S2X VL-SNR:
    McVlsnrSet1 = 0x40,
    McVlsnrSet2 = 0x41,
    // DVB-S2X NORMAL:
    McQpsk13_45 = 0x42,
    McQpsk9_20,
    McQpsk11_20,
    Mc8apsk5_9L,
    Mc8apsk26_45L,
    Mc8psk23_36,
    Mc8psk25_36,
    Mc8psk13_18,
    Mc16apsk1_2L,
    Mc16apsk8_15L,
    Mc16apsk5_9L,
    Mc16apsk26_45,
    Mc16apsk3_5,
    Mc16apsk3_5L,
    Mc16apsk28_45,
    Mc16apsk23_36,
    Mc16apsk2_3L,
    Mc16apsk25_36,
    Mc16apsk13_18,
    Mc16apsk7_9,
    Mc16apsk77_90,
    Mc32apsk2_3L,
    Mc32apsk32_45,
    Mc32apsk11_15,
    Mc32apsk7_9,
    Mc64apsk32_45L,
    Mc64apsk11_15,
    Mc64apsk7_9,
    Mc64apsk4_5,
    Mc64apsk5_6,
    Mc128apsk3_4,
    Mc128apsk7_9,
    Mc256apsk29_45L,
    Mc256apsk2_3L,
    Mc256apsk31_45L,
    Mc256apsk32_45,
    Mc256apsk11_15L,
    Mc256apsk3_4,
    // DVB-S2X SHORT:
    McQpsk11_45S,
    McQpsk4_15S,
    McQpsk14_45S,
    McQpsk7_15S,
    McQpsk8_15S,
    McQpsk32_45S,
    Mc8psk7_15S,
    Mc8psk8_15S,
    Mc8psk26_45S,
    Mc8psk32_45S,
    Mc16apsk7_15S,
    Mc16apsk8_15S,
    Mc16apsk26_45S,
    Mc16apsk3_5S,
    Mc16apsk32_45S,
    Mc32apsk2_3S,
    Mc32apsk32_45S,
}

#[derive(Clone, Copy)]
pub enum VlsnrHeader {
    NormalQpsk2_9 = 0,
    MediumBpsk1_5,
    MediumBpsk11_45,
    MediumBpsk1_3,
    ShortBpskSf2_1_5,
    ShortBpskSf2_11_45,
    ShortBpsk1_5 = 9,
    ShortBpsk4_15,
    ShortBpsk1_3,
    Dummy,
}

pub const NORMAL_PUNCTURING: usize = 3240;
pub const MEDIUM_PUNCTURING: usize = 1620;
pub const SHORT_PUNCTURING_SET1: usize = 810;
pub const SHORT_PUNCTURING_SET2: usize = 1224;

pub const EXTRA_PILOT_SYMBOLS_SET1: usize = (18 * 34) + (3 * 36);
pub const EXTRA_PILOT_SYMBOLS_SET2: usize = (9 * 32) + 36;

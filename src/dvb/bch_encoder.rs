use futuresdr::anyhow::Result;
use futuresdr::async_trait::async_trait;
use futuresdr::runtime::{
    Block, BlockMeta, BlockMetaBuilder, Kernel, MessageIo, MessageIoBuilder, StreamIo,
    StreamIoBuilder, WorkIo,
};

use super::config::{CodeRate, Framesize};

fn poly_mult(ina: &[u8], inb: &[u8]) -> Vec<u8> {
    let mut out = vec![0; ina.len() + inb.len()];
    for i in 0..ina.len() {
        for j in 0..inb.len() {
            if ina[i] * inb[j] > 0 {
                out[i + j] ^= 1;
            }
        }
    }
    if let Some(last) = out.iter().rposition(|&i| i != 0) {
        out.truncate(last + 1);
    }
    out
}
trait VecPolyExt {
    fn mult(&self, other: &[u8]) -> Vec<u8>;
}
impl VecPolyExt for Vec<u8> {
    fn mult(&self, other: &[u8]) -> Vec<u8> {
        poly_mult(self, other)
    }
}

const NUM_PARITY_N12: usize = 192;
const NUM_PARITY_N10: usize = 160;
const NUM_PARITY_N8: usize = 128;
const NUM_PARITY_S12: usize = 168;
const NUM_PARITY_M12: usize = 180;
const POLY_SIZE_N12: usize = NUM_PARITY_N12 / 32 + if NUM_PARITY_N12 % 32 != 0 { 1 } else { 0 };
const POLY_SIZE_N10: usize = NUM_PARITY_N10 / 32 + if NUM_PARITY_N10 % 32 != 0 { 1 } else { 0 };
const POLY_SIZE_N8: usize = NUM_PARITY_N8 / 32 + if NUM_PARITY_N8 % 32 != 0 { 1 } else { 0 };
const POLY_SIZE_S12: usize = NUM_PARITY_S12 / 32 + if NUM_PARITY_S12 % 32 != 0 { 1 } else { 0 };
const POLY_SIZE_M12: usize = NUM_PARITY_M12 / 32 + if NUM_PARITY_M12 % 32 != 0 { 1 } else { 0 };

enum BchCode {
    Normal12 {
        k: usize,
        n: usize,
        poly: [u32; POLY_SIZE_N12],
    },
    Normal10 {
        k: usize,
        n: usize,
        poly: [u32; POLY_SIZE_N10],
    },
    Normal8 {
        k: usize,
        n: usize,
        poly: [u32; POLY_SIZE_N8],
    },
    Short12 {
        k: usize,
        n: usize,
        poly: [u32; POLY_SIZE_S12],
    },
    Medium12 {
        k: usize,
        n: usize,
        poly: [u32; POLY_SIZE_M12],
    },
}

/// ### TODO: optimize bit operations
fn poly_pack(poly: Vec<u8>) -> Vec<u32> {
    poly.chunks(32)
        .map(|chunk| {
            let mut out = 0;
            let mut bit = 0x80000000;
            for &j in chunk {
                if j != 0 {
                    out |= bit;
                }
                bit >>= 1;
            }
            out
        })
        .collect()
}

impl BchCode {
    fn new_n_12(n: usize) -> Self {
        let poly = poly_mult(
            &[1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            &[1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
        )
        .mult(&[1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1])
        .mult(&[1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1])
        .mult(&[1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1])
        .mult(&[1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1])
        .mult(&[1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1])
        .mult(&[1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1])
        .mult(&[1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1])
        .mult(&[1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1])
        .mult(&[1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1])
        .mult(&[1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1]);

        let poly = poly_pack(poly);
        assert_eq!(poly.len(), POLY_SIZE_N12);

        Self::Normal12 {
            k: n - NUM_PARITY_N12,
            n,
            poly: poly.try_into().unwrap(),
        }
    }

    fn new_n_10(n: usize) -> Self {
        let poly = poly_mult(
            &[1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            &[1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
        )
        .mult(&[1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1])
        .mult(&[1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1])
        .mult(&[1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1])
        .mult(&[1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1])
        .mult(&[1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1])
        .mult(&[1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1])
        .mult(&[1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1])
        .mult(&[1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1]);

        let poly = poly_pack(poly);
        assert_eq!(poly.len(), POLY_SIZE_N10);

        Self::Normal10 {
            k: n - NUM_PARITY_N10,
            n,
            poly: poly.try_into().unwrap(),
        }
    }

    fn new_n_8(n: usize) -> Self {
        let poly = poly_mult(
            &[1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            &[1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
        )
        .mult(&[1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1])
        .mult(&[1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1])
        .mult(&[1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1])
        .mult(&[1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1])
        .mult(&[1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1])
        .mult(&[1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1]);

        let poly = poly_pack(poly);
        assert_eq!(poly.len(), POLY_SIZE_N8);

        Self::Normal8 {
            k: n - NUM_PARITY_N8,
            n,
            poly: poly.try_into().unwrap(),
        }
    }

    fn new_s_12(n: usize) -> Self {
        let poly = poly_mult(
            &[1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            &[1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1],
        )
        .mult(&[1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1])
        .mult(&[1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1])
        .mult(&[1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1])
        .mult(&[1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1])
        .mult(&[1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1])
        .mult(&[1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1])
        .mult(&[1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1])
        .mult(&[1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1])
        .mult(&[1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1])
        .mult(&[1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1]);

        let poly = poly_pack(poly);
        assert_eq!(poly.len(), POLY_SIZE_S12);

        Self::Short12 {
            k: n - NUM_PARITY_S12,
            n,
            poly: poly.try_into().unwrap(),
        }
    }

    fn new_m_12(n: usize) -> Self {
        let poly = poly_mult(
            &[1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            &[1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1],
        )
        .mult(&[1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1])
        .mult(&[1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1])
        .mult(&[1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1])
        .mult(&[1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1])
        .mult(&[1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1])
        .mult(&[1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1])
        .mult(&[1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1])
        .mult(&[1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1])
        .mult(&[1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1])
        .mult(&[1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1]);

        let poly = poly_pack(poly);
        assert_eq!(poly.len(), POLY_SIZE_M12);

        Self::Medium12 {
            k: n - NUM_PARITY_M12,
            n,
            poly: poly.try_into().unwrap(),
        }
    }

    pub fn dvb(framesize: Framesize, rate: CodeRate) -> Option<Self> {
        Some(match framesize {
            Framesize::Normal => match rate {
                CodeRate::C1_4 => Self::new_n_12(16200),
                CodeRate::C1_3 => Self::new_n_12(21600),
                CodeRate::C2_5 => Self::new_n_12(25920),
                CodeRate::C1_2 => Self::new_n_12(32400),
                CodeRate::C3_5 => Self::new_n_12(38880),
                CodeRate::C2_3 => Self::new_n_10(43200),
                CodeRate::C3_4 => Self::new_n_12(48600),
                CodeRate::C4_5 => Self::new_n_12(51840),
                CodeRate::C5_6 => Self::new_n_10(54000),
                CodeRate::C8_9 => Self::new_n_8(57600),
                CodeRate::C9_10 => Self::new_n_8(58320),
                CodeRate::C13_45 => Self::new_n_12(18720),
                CodeRate::C9_20 => Self::new_n_12(29160),
                CodeRate::C90_180 => Self::new_n_12(32400),
                CodeRate::C96_180 => Self::new_n_12(34560),
                CodeRate::C11_20 => Self::new_n_12(35640),
                CodeRate::C100_180 => Self::new_n_12(36000),
                CodeRate::C104_180 => Self::new_n_12(37440),
                CodeRate::C26_45 => Self::new_n_12(37440),
                CodeRate::C18_30 => Self::new_n_12(38880),
                CodeRate::C28_45 => Self::new_n_12(40320),
                CodeRate::C23_36 => Self::new_n_12(41400),
                CodeRate::C116_180 => Self::new_n_12(41760),
                CodeRate::C20_30 => Self::new_n_12(43200),
                CodeRate::C124_180 => Self::new_n_12(44640),
                CodeRate::C25_36 => Self::new_n_12(45000),
                CodeRate::C128_180 => Self::new_n_12(46080),
                CodeRate::C13_18 => Self::new_n_12(46800),
                CodeRate::C132_180 => Self::new_n_12(47520),
                CodeRate::C22_30 => Self::new_n_12(47520),
                CodeRate::C135_180 => Self::new_n_12(48600),
                CodeRate::C140_180 => Self::new_n_12(50400),
                CodeRate::C7_9 => Self::new_n_12(50400),
                CodeRate::C154_180 => Self::new_n_12(55440),
                CodeRate::C2_9Vlsnr => Self::new_n_12(14400),
                _ => return None,
            },
            Framesize::Short => match rate {
                CodeRate::C1_4 => Self::new_s_12(3240),
                CodeRate::C1_3 => Self::new_s_12(5400),
                CodeRate::C2_5 => Self::new_s_12(6480),
                CodeRate::C1_2 => Self::new_s_12(7200),
                CodeRate::C3_5 => Self::new_s_12(9720),
                CodeRate::C2_3 => Self::new_s_12(10800),
                CodeRate::C3_4 => Self::new_s_12(11880),
                CodeRate::C4_5 => Self::new_s_12(12600),
                CodeRate::C5_6 => Self::new_s_12(13320),
                CodeRate::C8_9 => Self::new_s_12(14400),
                CodeRate::C11_45 => Self::new_s_12(3960),
                CodeRate::C4_15 => Self::new_s_12(4320),
                CodeRate::C14_45 => Self::new_s_12(5040),
                CodeRate::C7_15 => Self::new_s_12(7560),
                CodeRate::C8_15 => Self::new_s_12(8640),
                CodeRate::C26_45 => Self::new_s_12(9360),
                CodeRate::C32_45 => Self::new_s_12(11520),
                CodeRate::C1_5VlsnrSf2 => Self::new_s_12(2680),
                CodeRate::C11_45VlsnrSf2 => Self::new_s_12(3960),
                CodeRate::C1_5Vlsnr => Self::new_s_12(3240),
                CodeRate::C4_15Vlsnr => Self::new_s_12(4320),
                CodeRate::C1_3Vlsnr => Self::new_s_12(5400),
                _ => return None,
            },
            Framesize::Medium => match rate {
                CodeRate::C1_5Medium => Self::new_m_12(5840),
                CodeRate::C11_45Medium => Self::new_m_12(7920),
                CodeRate::C1_3Medium => Self::new_m_12(10800),
                _ => return None,
            },
        })
    }
}

struct ShiftReg<const SIZE: usize>([u32; SIZE]);

impl<const SIZE: usize> ShiftReg<SIZE> {
    pub fn new() -> Self {
        Self([0; SIZE])
    }

    pub fn xor(&mut self, other: &[u32]) {
        // self.0.iter_mut().zip(other.iter()).for_each(|(s, p)| *s ^= *p);
        if other.len() >= SIZE {
            for i in 0..SIZE {
                self.0[i] ^= other[i];
            }
        }
    }

    pub fn shift(&mut self) {
        for i in (1..SIZE).rev() {
            self.0[i] = (self.0[i] >> 1) | (self.0[i - 1] << 31);
        }
        self.0[0] >>= 1;
    }

    pub fn peek(&self) -> u32 {
        *self.0.last().unwrap()
    }
}

pub struct BchEncoder(BchCode);

impl BchEncoder {
    pub fn new(framesize: Framesize, rate: CodeRate) -> Option<Block> {
        Some(Block::new(
            BlockMetaBuilder::new("DVB_BCH_Encoder").build(),
            StreamIoBuilder::new()
                .add_input::<u8>("in")
                .add_output::<u8>("out")
                .build(),
            MessageIoBuilder::<Self>::new().build(),
            Self(BchCode::dvb(framesize, rate)?),
        ))
    }

    fn bch_work<const SIZE: usize>(
        sio: &mut StreamIo,
        k: usize,
        n: usize,
        poly: &[u32],
    ) -> Result<bool> {
        let i = sio.input(0).slice::<u8>();
        let o = sio.output(0).slice::<u8>();
        let mut shreg = ShiftReg::<SIZE>::new();

        let m = std::cmp::min(i.len() / k, o.len() / n);
        if m > 0 {
            for (v, r) in i.chunks_exact(k).zip(o.chunks_exact_mut(n)) {
                let (info, par) = r.split_at_mut(k);
                info.copy_from_slice(v);

                for b in v {
                    let b = b ^ (shreg.peek() & 1) as u8;
                    shreg.shift();
                    if b != 0 {
                        shreg.xor(poly);
                    }
                }
                for p in par {
                    *p = (shreg.peek() & 1) as u8;
                    shreg.shift();
                }
            }

            sio.input(0).consume(k * m);
            sio.output(0).produce(n * m);
        }

        Ok(sio.input(0).finished() && m == i.len())
    }
}

#[async_trait]
impl Kernel for BchEncoder {
    async fn work(
        &mut self,
        io: &mut WorkIo,
        sio: &mut StreamIo,
        _mio: &mut MessageIo<Self>,
        _meta: &mut BlockMeta,
    ) -> Result<()> {
        io.finished = match self.0 {
            BchCode::Normal12 { k, n, poly }
            | BchCode::Short12 { k, n, poly }
            | BchCode::Medium12 { k, n, poly } => Self::bch_work::<6>(sio, k, n, &poly)?,
            BchCode::Normal10 { k, n, poly } => Self::bch_work::<5>(sio, k, n, &poly)?,
            BchCode::Normal8 { k, n, poly } => Self::bch_work::<4>(sio, k, n, &poly)?,
        };
        Ok(())
    }
}

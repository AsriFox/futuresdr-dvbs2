mod dvb;
mod random_src;

use futuresdr::anyhow::Result;
use futuresdr::blocks::{ApplyNM, ConsoleSink, Head};
use futuresdr::macros::connect;
use futuresdr::runtime::{Flowgraph, Runtime};

fn main() -> Result<()> {
    let mut fg = Flowgraph::new();

    let framesize = dvb::config::Framesize::Normal;
    let rate = dvb::config::CodeRate::C1_4;
    let constellation = dvb::config::Constellation::ModQpsk;

    let src = random_src::RandomByteSource::new();
    let unpack = ApplyNM::<_, u8, u8, 1, 8>::new(move |bytes, bits| {
        let b = bytes[0];
        for i in 0..8 {
            bits[i] = (b >> i) & 1;
        }
    });
    let head = Head::<u8>::new(16008);
    let bch = dvb::BchEncoder::new(framesize, rate).unwrap();
    let snk = ConsoleSink::<u8>::new("");

    connect!(fg, src > unpack > head > bch > snk);

    Runtime::new().run(fg)?;

    Ok(())
}

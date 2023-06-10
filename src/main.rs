use futuresdr::anyhow::Result;
use futuresdr::blocks::{ConsoleSink, Head, NullSource};
use futuresdr::macros::connect;
use futuresdr::runtime::{Flowgraph, Runtime};

fn main() -> Result<()> {
    let mut fg = Flowgraph::new();

    let src = NullSource::<u8>::new();
    let head = Head::<u8>::new(1024);
    let snk = ConsoleSink::<u8>::new("");

    connect!(fg, src > head > snk);

    Runtime::new().run(fg)?;

    Ok(())
}

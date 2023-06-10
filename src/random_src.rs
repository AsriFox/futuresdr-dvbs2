use fastrand::Rng;
use futuresdr::anyhow::Result;
use futuresdr::async_trait::async_trait;
use futuresdr::runtime::{
    Block, BlockMeta, BlockMetaBuilder, Kernel, MessageIo, MessageIoBuilder, StreamIo,
    StreamIoBuilder, WorkIo,
};

pub struct RandomByteSource {
    rng: Rng,
}

impl RandomByteSource {
    pub fn new() -> Block {
        Block::new(
            BlockMetaBuilder::new("RandomByteSource").build(),
            StreamIoBuilder::new().add_output::<u8>("out").build(),
            MessageIoBuilder::<Self>::new().build(),
            Self { rng: Rng::new() },
        )
    }
}

#[async_trait]
impl Kernel for RandomByteSource {
    async fn work(
        &mut self,
        _io: &mut WorkIo,
        sio: &mut StreamIo,
        _mio: &mut MessageIo<Self>,
        _meta: &mut BlockMeta,
    ) -> Result<()> {
        let o = sio.output(0).slice();

        self.rng.fill(o);

        sio.output(0).produce(o.len());

        Ok(())
    }
}

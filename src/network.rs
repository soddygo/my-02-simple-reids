use crate::{Backend, RespFrame};
use anyhow::Result;
use futures::SinkExt;

use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder, Framed};
use tracing::info;

#[derive(Debug)]
struct RespFrameCodec;

#[derive(Debug)]
struct RedisRequest {
    frame: RespFrame,
    backend: Backend,
}

#[derive(Debug)]
struct RedisResponse {
    frame: RespFrame,
}


pub async fn stream_handler(stream: TcpStream, backend: Backend) -> Result<()> {
    let mut framed = Framed::new(stream, RespFrameCodec);
    loop {
        match framed.next().await {
            Some(Ok(frame)) => {
                info!("recv frame:{:?}",frame);
                let req = RedisRequest {
                    frame,
                    backend: backend.clone(),
                };

                let resp = request_handle(req).await?;
                info!("resp:{:?}",resp.frame);
                framed.send(resp).await?;
            }
            Some(Err(err)) => return Err(err),
            None => return Ok(())
        }
    }
}


async fn request_handle(req: RedisRequest) -> Result<RedisResponse> {
    let (frame, backend) = (req.frame, req.backend);
    todo!()
}

impl Encoder<RespFrame> for RespFrameCodec{
    type Error = anyhow::Error;

    fn encode(&mut self, item: RespFrame, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        todo!()
    }
}


impl Decoder for RespFrameCodec{
    type Item = RespFrame;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}
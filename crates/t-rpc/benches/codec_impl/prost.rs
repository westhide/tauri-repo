use bytes::BytesMut;
use prost::Message;
use tonic::codec::{
    Codec as ICodec, DecodeBuf, Decoder as IDecoder, EncodeBuf, Encoder as IEncoder, ProstCodec,
};

const DEFAULT_CODEC_BUFFER_SIZE: usize = 8 * 1024;

pub fn roundtrip<T>(pt: T)
where
    T: Send + 'static,
    T: Message + Default,
{
    let mut codec = ProstCodec::<T, T>::default();
    let mut enc = codec.encoder();
    let mut buf = BytesMut::with_capacity(DEFAULT_CODEC_BUFFER_SIZE);
    enc.encode(pt, &mut EncodeBuf::new(&mut buf)).unwrap();
    pt.encode(&mut buf);
    let mut dec = codec.decoder();
    let len = buf.len();
    dec.decode(&mut DecodeBuf::new(&mut buf, len)).unwrap();
}

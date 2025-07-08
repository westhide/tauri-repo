use bytes::BytesMut;
use rkyv::{Archive, Deserialize, Serialize};
use t_rpc::codec::rkyv::{Codec, DSerializer, ESerializer};
use tonic::codec::{
    Codec as ICodec, DecodeBuf, Decoder as IDecoder, EncodeBuf, Encoder as IEncoder,
};

const DEFAULT_CODEC_BUFFER_SIZE: usize = 8 * 1024;

pub fn roundtrip<T>(pt: T)
where
    T: Send + 'static,
    T: for<'a, 'b, 'c> Serialize<ESerializer<'a, 'b, 'c>>,
    T: Archive,
    T::Archived: Deserialize<T, DSerializer>,
{
    let mut codec = Codec::<T, T>::default();
    let mut enc = codec.encoder();
    let mut buf = BytesMut::with_capacity(DEFAULT_CODEC_BUFFER_SIZE);
    enc.encode(pt, &mut EncodeBuf::new(&mut buf)).unwrap();
    let mut dec = codec.decoder();
    let len = buf.len();
    dec.decode(&mut DecodeBuf::new(&mut buf, len)).unwrap();
}

use std::marker::PhantomData;

use nill::{Nil, nil};
use prost::bytes::{Buf, BufMut};
use rkyv::{
    Archive, Deserialize, Serialize,
    api::high::{HighDeserializer, HighSerializer, to_bytes_in},
    deserialize,
    rancor::Error,
    ser::{allocator::ArenaHandle, writer::IoWriter},
};
use tonic::{
    Status,
    codec::{Codec as ICodec, DecodeBuf, Decoder as IDecoder, EncodeBuf, Encoder as IEncoder},
};

#[derive(Debug, Default)]
pub struct Codec<T, U> {
    maker: PhantomData<(T, U)>,
}

impl<T, U> ICodec for Codec<T, U>
where
    T: Send + 'static,
    U: Send + 'static,
    T: for<'a> Serialize<HighSerializer<Vec<u8>, ArenaHandle<'a>, Error>>,
    U: Archive,
    U::Archived: Deserialize<U, HighDeserializer<Error>>,
{
    type Decode = U;
    type Decoder = Decoder<U>;
    type Encode = T;
    type Encoder = Encoder<T>;

    fn encoder(&mut self) -> Self::Encoder {
        Self::Encoder::new()
    }

    fn decoder(&mut self) -> Self::Decoder {
        Self::Decoder::new()
    }
}

#[derive(Debug)]
pub struct Encoder<T> {
    marker: PhantomData<T>,
}

impl<T> Encoder<T> {
    pub fn new() -> Self {
        Self { marker: PhantomData }
    }
}

impl<T> IEncoder for Encoder<T>
where
    T: for<'a> Serialize<HighSerializer<Vec<u8>, ArenaHandle<'a>, Error>>,
{
    type Error = Status;
    type Item = T;

    fn encode(&mut self, item: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<Nil, Self::Error> {
        let bytes = Vec::new();
        let src = to_bytes_in(&item, bytes).unwrap();
        dst.put_slice(&src);
        Ok(nil)
    }
}

#[derive(Debug, Default)]
pub struct Decoder<U> {
    marker: PhantomData<U>,
}

impl<U> Decoder<U> {
    pub fn new() -> Self {
        Self { marker: PhantomData }
    }
}

impl<U> IDecoder for Decoder<U>
where
    U: Archive,
    U::Archived: Deserialize<U, HighDeserializer<Error>>,
{
    type Error = Status;
    type Item = U;

    fn decode(&mut self, src: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        let archived = unsafe { rkyv::access_unchecked::<U::Archived>(src.chunk()) };
        let deserialized = deserialize::<U, Error>(archived).unwrap();
        Ok(Some(deserialized))
    }
}

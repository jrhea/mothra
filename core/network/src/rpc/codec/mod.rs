pub(crate) mod base;
pub(crate) mod serenity;

use self::base::{BaseInboundCodec, BaseOutboundCodec};
use self::serenity::{SerenityInboundCodec, SerenityOutboundCodec};
use crate::rpc::protocol::RPCError;
use crate::rpc::{RPCErrorResponse, RPCRequest};
use bytes::BytesMut;
use tokio::codec::{Decoder, Encoder};

// Known types of codecs
pub enum InboundCodec {
    Serenity(BaseInboundCodec<SerenityInboundCodec>),
}

pub enum OutboundCodec {
    Serenity(BaseOutboundCodec<SerenityOutboundCodec>),
}

impl Encoder for InboundCodec {
    type Item = RPCErrorResponse;
    type Error = RPCError;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match self {
            InboundCodec::Serenity(codec) => codec.encode(item, dst),
        }
    }
}

impl Decoder for InboundCodec {
    type Item = RPCRequest;
    type Error = RPCError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self {
            InboundCodec::Serenity(codec) => codec.decode(src),
        }
    }
}

impl Encoder for OutboundCodec {
    type Item = RPCRequest;
    type Error = RPCError;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match self {
            OutboundCodec::Serenity(codec) => codec.encode(item, dst),
        }
    }
}

impl Decoder for OutboundCodec {
    type Item = RPCErrorResponse;
    type Error = RPCError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self {
            OutboundCodec::Serenity(codec) => codec.decode(src),
        }
    }
}

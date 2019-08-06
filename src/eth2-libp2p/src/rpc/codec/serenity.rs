use crate::rpc::methods::*;
use crate::rpc::{
    codec::base::OutboundCodec,
    protocol::{ProtocolId, RPCError},
};
use crate::rpc::{ErrorMessage, RPCErrorResponse, RPCRequest, RPCResponse};
use bytes::{Bytes, BytesMut};
use tokio::codec::{Decoder, Encoder};
use unsigned_varint::codec::UviBytes;

/* Inbound Codec */

pub struct SerenityInboundCodec {
    inner: UviBytes,
    protocol: ProtocolId,
}

impl SerenityInboundCodec {
    pub fn new(protocol: ProtocolId, max_packet_size: usize) -> Self {
        let mut uvi_codec = UviBytes::default();
        uvi_codec.set_max_len(max_packet_size);

        // this encoding only applies to Serenity.
        debug_assert!(protocol.encoding.as_str() == "ssz");

        SerenityInboundCodec {
            inner: uvi_codec,
            protocol,
        }
    }
}

// Encoder for inbound
impl Encoder for SerenityInboundCodec {
    type Item = RPCErrorResponse;
    type Error = RPCError;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = match item {

            RPCErrorResponse::Success(resp) => {
                match resp {
                    RPCResponse::Hello(res) => Bytes::from(res.value),
                }
            },
            RPCErrorResponse::InvalidRequest(err) => Bytes::from(err.as_string()),
            RPCErrorResponse::ServerError(err) => Bytes::from(err.as_string()),
            RPCErrorResponse::Unknown(err) => Bytes::from(err.as_string()),
        };

        if !bytes.is_empty() {
            // length-prefix and return
            return self
                .inner
                .encode(Bytes::from(bytes), dst)
                .map_err(RPCError::from);
        }
        Ok(())
    }
}

// Decoder for inbound
impl Decoder for SerenityInboundCodec {
    type Item = RPCRequest;
    type Error = RPCError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.inner.decode(src).map_err(RPCError::from) {
            Ok(Some(_packet)) => match self.protocol.message_name.as_str() {
                "hello" => match self.protocol.version.as_str() {
                    "1.0.0" => Ok(Some(RPCRequest::Hello(HelloMessage{
                        value: String::from_utf8(_packet.to_vec()).unwrap(),
                    }
                    ))),
                    _ => Err(RPCError::InvalidProtocol("Unknown HELLO version")),
                },
                _ => Err(RPCError::InvalidProtocol("Unknown message name.")),
            },
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

/* Outbound Codec */

pub struct SerenityOutboundCodec {
    inner: UviBytes,
    protocol: ProtocolId,
}

impl SerenityOutboundCodec {
    pub fn new(protocol: ProtocolId, max_packet_size: usize) -> Self {
        let mut uvi_codec = UviBytes::default();
        uvi_codec.set_max_len(max_packet_size);

        // this encoding only applies to Serenity.
        debug_assert!(protocol.encoding.as_str() == "ssz");

        SerenityOutboundCodec {
            inner: uvi_codec,
            protocol,
        }
    }
}

// Encoder for outbound
impl Encoder for SerenityOutboundCodec {
    type Item = RPCRequest;
    type Error = RPCError;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = match item {
            RPCRequest::Hello(req) => Bytes::from(req.value),
        };
        // length-prefix
        self.inner
            .encode(bytes::Bytes::from(bytes), dst)
            .map_err(RPCError::from)
    }
}

// Decoder for outbound
impl Decoder for SerenityOutboundCodec {
    type Item = RPCResponse;
    type Error = RPCError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.inner.decode(src).map_err(RPCError::from) {
            Ok(Some(_packet)) => match self.protocol.message_name.as_str() {
                "hello" => match self.protocol.version.as_str() {
                    
                    "1.0.0" => Ok(Some(RPCResponse::Hello(HelloMessage{
                        value: String::from_utf8(_packet.to_vec()).unwrap(),
                    }
                    ))),
                    _ => Err(RPCError::InvalidProtocol("Unknown HELLO version.")),
                },
                _ => Err(RPCError::InvalidProtocol("Unknown method")),
            },
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

impl OutboundCodec for SerenityOutboundCodec {
    type ErrorType = ErrorMessage;

    fn decode_error(&mut self, src: &mut BytesMut) -> Result<Option<Self::ErrorType>, RPCError> {
        match self.inner.decode(src).map_err(RPCError::from) {
            Ok(Some(_packet)) => Ok(None),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

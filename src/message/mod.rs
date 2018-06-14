use std::error::Error;
use std::io::Cursor;
use std::io;

use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};

mod api;
mod p2p;

/// The different message types supported by this module
///
/// For each message type, there exists a corresponding
/// struct holding the contents of this message.

pub enum Message {
    DhtPut(api::DhtPut),
    DhtGet(api::DhtGet),
    DhtSuccess(api::DhtSuccess),
    DhtFailure(api::DhtFailure),
    StorageGet(p2p::StorageGet),
    StoragePut(p2p::StoragePut),
    StorageGetSuccess(p2p::StorageGetSuccess),
    StoragePutSuccess(p2p::StoragePutSuccess),
    StorageFailure(p2p::StorageFailure),
    PeerFind(p2p::PeerFind),
    PeerFound(p2p::PeerFound),
    PredecessorGet(p2p::PredecessorGet),
    PredecessorReply(p2p::PredecessorReply),
    PredecessorSet(p2p::PredecessorSet)
}

impl Message {
    const DHT_PUT: u16 = 650;
    const DHT_GET: u16 = 651;
    const DHT_SUCCESS: u16 = 652;
    const DHT_FAILURE: u16 = 653;

    const STORAGE_GET: u16 = 1000;
    const STORAGE_PUT: u16 = 1001;
    const STORAGE_GET_SUCESS: u16 = 1002;
    const STORAGE_PUT_SUCCESS: u16 = 1003;
    const STORAGE_FAILURE: u16 = 1004;

    const PEER_FIND: u16 = 1050;
    const PEER_FOUND: u16 = 1051;
    const PREDECESSOR_GET: u16 = 1052;
    const PREDECESSOR_REPLY: u16 = 1053;
    const PREDECESSOR_SET: u16 = 1054;

    pub fn new(buffer: &[u8]) -> io::Result<Self> {
        let mut cursor = Cursor::new(buffer);
        let size = cursor.read_u16::<NetworkEndian>()? as usize;
        let msg_type = cursor.read_u16::<NetworkEndian>()?;

        if buffer.len() != size {
            // todo define own error type
            return Err(io::Error::new(io::ErrorKind::Other, "Non-matching message size"))
        }

        let msg = match msg_type {
            DHT_PUT =>
                Message::DhtPut(api::DhtPut::parse(cursor)?),
            DHT_GET =>
                Message::DhtGet(api::DhtGet::parse(cursor)?),
            DHT_SUCCESS =>
                Message::DhtSuccess(api::DhtSuccess::parse(cursor)?),
            DHT_FAILURE =>
                Message::DhtFailure(api::DhtFailure::parse(cursor)?),
            STORAGE_GET =>
                Message::StorageGet(p2p::StorageGet::parse(cursor)?),
            STORAGE_PUT =>
                Message::StoragePut(p2p::StoragePut::parse(cursor)?),
            STORAGE_GET_SUCCESS =>
                Message::StorageGetSuccess(p2p::StorageGetSuccess::parse(cursor)?),
            STORAGE_PUT_SUCCESS =>
                Message::StoragePutSuccess(p2p::StoragePutSuccess::parse(cursor)?),
            STORAGE_FAILURE =>
                Message::StorageFailure(p2p::StorageFailure::parse(cursor)?),
            PEER_FIND =>
                Message::PeerFind(p2p::PeerFind::parse(cursor)?),
            PEEER_FOUND =>
                Message::PeerFound(p2p::PeerFound::parse(cursor)?),
            PREDECESSOR_GET =>
                Message::PredecessorGet(p2p::PredecessorGet::parse(cursor)?),
            PREDECESSOR_REPLY =>
                Message::PredecessorReply(p2p::PredecessorReply::parse(cursor)?),
            PREDECESSOR_SET =>
                Message::PredecessorSet(p2p::PredecessorSet::parse(cursor)?),
            _ =>
                // todo define own Error type
                return Err(io::Error::new(io::ErrorKind::Other, "Invalid message type"))
        };

        Ok(msg)
    }

    pub fn write_bytes(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        panic!("not implemented")
    }
}

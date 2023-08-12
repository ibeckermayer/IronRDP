//! PDUs for [[MS-RDPEFS]: Remote Desktop Protocol: File System Virtual Channel Extension](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpefs/34d9de58-b2b5-40b6-b970-f82d4603bdb5)

use std::marker::PhantomData;
use std::{fmt::Debug, fmt::Formatter};

use crate::{
    cursor::{ReadCursor, WriteCursor},
    Pdu, PduDecode, PduEncode, PduResult,
};
use num_traits::FromPrimitive;

/// [2.2.1.1 Shared Header (RDPDR_HEADER)](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpefs/29d4108f-8163-4a67-8271-e48c4b9c2a7c)
#[derive(Debug)]
pub struct SharedHeader {
    pub component: Component,
    pub packet_id: PacketId,
}

impl Pdu for SharedHeader {
    const NAME: &'static str = "SharedHeader";
}

impl<'de> PduDecode<'de> for SharedHeader {
    fn decode(src: &mut ReadCursor<'de>) -> PduResult<Self> {
        Ok(Self {
            component: Component::from_u16(src.read_u16())
                .ok_or_else(|| invalid_message_err!("Component", "invalid value"))?,
            packet_id: PacketId::from_u16(src.read_u16())
                .ok_or_else(|| invalid_message_err!("PacketId", "invalid value"))?,
        })
    }
}

#[derive(Debug, FromPrimitive)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum Component {
    RDPDR_CTYP_CORE = 0x4472,
    RDPDR_CTYP_PRN = 0x5052,
}

#[derive(Debug, FromPrimitive)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum PacketId {
    PAKID_CORE_SERVER_ANNOUNCE = 0x496E,
    PAKID_CORE_CLIENTID_CONFIRM = 0x4343,
    PAKID_CORE_CLIENT_NAME = 0x434E,
    PAKID_CORE_DEVICELIST_ANNOUNCE = 0x4441,
    PAKID_CORE_DEVICE_REPLY = 0x6472,
    PAKID_CORE_DEVICE_IOREQUEST = 0x4952,
    PAKID_CORE_DEVICE_IOCOMPLETION = 0x4943,
    PAKID_CORE_SERVER_CAPABILITY = 0x5350,
    PAKID_CORE_CLIENT_CAPABILITY = 0x4350,
    PAKID_CORE_DEVICELIST_REMOVE = 0x444D,
    PAKID_PRN_CACHE_DATA = 0x5043,
    PAKID_CORE_USER_LOGGEDON = 0x554C,
    PAKID_PRN_USING_XPS = 0x5543,
}

/// VersionAndIdPDU is a fixed size structure representing multiple PDUs.
/// See [ServerAnnounceRequest] for an example of how it's to be used.
pub struct VersionAndIdPDU<T: Pdu> {
    version_major: u16,
    version_minor: u16,
    pub client_id: u32,
    _phantom: PhantomData<T>,
}

impl<T: Pdu> VersionAndIdPDU<T> {
    pub fn new(version_major: u16, version_minor: u16, client_id: u32) -> Self {
        Self {
            version_major,
            version_minor,
            client_id,
            _phantom: PhantomData,
        }
    }
}

impl<T: Pdu> Debug for VersionAndIdPDU<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct(T::NAME)
            .field("version_major", &self.version_major)
            .field("version_minor", &self.version_minor)
            .field("client_id", &self.client_id)
            .finish()
    }
}

impl<'de, T> PduDecode<'de> for VersionAndIdPDU<T>
where
    T: Pdu,
{
    fn decode(src: &mut ReadCursor<'de>) -> PduResult<Self> {
        Ok(Self {
            version_major: src.read_u16(),
            version_minor: src.read_u16(),
            client_id: src.read_u32(),
            _phantom: PhantomData,
        })
    }
}

impl<T> PduEncode for VersionAndIdPDU<T>
where
    T: Pdu,
{
    fn encode(&self, dst: &mut WriteCursor<'_>) -> PduResult<()> {
        dst.write_u16(self.version_major);
        dst.write_u16(self.version_minor);
        dst.write_u32(self.client_id);
        Ok(())
    }

    fn name(&self) -> &'static str {
        T::NAME
    }

    fn size(&self) -> usize {
        8 // u16 + u16 + u32 = 2 bytes + 2 bytes + 4 bytes = 8 bytes
    }
}

/// [2.2.2.2 Server Announce Request (DR_CORE_SERVER_ANNOUNCE_REQ)](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpefs/046047aa-62d8-49f9-bf16-7fe41880aaf4)
pub struct ServerAnnounceRequest_;
impl Pdu for ServerAnnounceRequest_ {
    const NAME: &'static str = "ServerAnnounceRequest";
}
pub type ServerAnnounceRequest = VersionAndIdPDU<ServerAnnounceRequest_>;

/// [2.2.2.3 Client Announce Reply (DR_CORE_CLIENT_ANNOUNCE_RSP)](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpefs/d6fe6d1b-c145-4a6f-99aa-4fe3cdcea398)
pub struct ClientAnnounceReply_;
impl Pdu for ClientAnnounceReply_ {
    const NAME: &'static str = "ServerAnnounceRequest";
}
pub type ClientAnnounceReply = VersionAndIdPDU<ClientAnnounceReply_>;

#[macro_use]
extern crate num_derive;

use ironrdp_pdu::{
    cursor::ReadCursor, gcc::ChannelName, invalid_message_err, write_buf::WriteBuf, Pdu, PduDecode, PduResult,
};
use ironrdp_svc::{CompressionCondition, StaticVirtualChannel};
use num_traits::FromPrimitive;
use tracing::{trace, warn};

/// The RDPDR channel as specified in [MS-RDPEFS].
///
/// This channel must always be advertised with the "rdpsnd"
/// channel in order for the server to send anything back to it,
/// see: https://tinyurl.com/2fvrtfjd.
#[derive(Debug)]
pub struct Rdpdr;

impl Default for Rdpdr {
    fn default() -> Self {
        Self::new()
    }
}

impl Rdpdr {
    pub const NAME: ChannelName = ChannelName::from_static(b"rdpdr\0\0\0");

    pub fn new() -> Self {
        Self
    }

    fn handle_server_announce(&mut self, payload: &mut ReadCursor<'_>, output: &mut WriteBuf) -> PduResult<()> {
        let request = ServerAnnounceRequest::decode(payload)?;
        trace!("{:?}", request);
        // TODO: send client announce reply
        Ok(())
    }
}

impl StaticVirtualChannel for Rdpdr {
    fn channel_name(&self) -> ChannelName {
        Self::NAME
    }

    fn compression_condition(&self) -> CompressionCondition {
        CompressionCondition::WhenRdpDataIsCompressed
    }

    fn process(&mut self, initiator_id: u16, channel_id: u16, payload: &[u8], output: &mut WriteBuf) -> PduResult<()> {
        let mut payload = ReadCursor::new(payload);

        let header = SharedHeader::decode(&mut payload)?;
        trace!("{:?}", header);

        if let Component::RDPDR_CTYP_PRN = header.component {
            warn!(
                "received {:?} RDPDR header from RDP server, printer redirection is unimplemented",
                Component::RDPDR_CTYP_PRN
            );
            return Ok(());
        }

        match header.packet_id {
            PacketId::PAKID_CORE_SERVER_ANNOUNCE => self.handle_server_announce(&mut payload, output)?,
            _ => {
                warn!("received unimplemented packet: {:?}", header.packet_id);
                return Ok(());
            }
        }

        warn!("received data, protocol is unimplemented");
        Ok(())
    }
}

/// [2.2.1.1 Shared Header (RDPDR_HEADER)](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpefs/29d4108f-8163-4a67-8271-e48c4b9c2a7c)
#[derive(Debug)]
struct SharedHeader {
    component: Component,
    packet_id: PacketId,
}

impl Pdu for SharedHeader {
    const NAME: &'static str = "SharedHeader";
}

impl<'de> PduDecode<'de> for SharedHeader {
    fn decode(src: &mut ironrdp_pdu::cursor::ReadCursor<'de>) -> PduResult<Self> {
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
enum Component {
    RDPDR_CTYP_CORE = 0x4472,
    RDPDR_CTYP_PRN = 0x5052,
}

#[derive(Debug, FromPrimitive)]
#[repr(u16)]
#[allow(non_camel_case_types)]
enum PacketId {
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

/// [2.2.2.2 Server Announce Request (DR_CORE_SERVER_ANNOUNCE_REQ)](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpefs/046047aa-62d8-49f9-bf16-7fe41880aaf4)
#[derive(Debug)]
struct ServerAnnounceRequest {
    version_major: u16,
    version_minor: u16,
    client_id: u32,
}

impl<'de> PduDecode<'de> for ServerAnnounceRequest {
    fn decode(src: &mut ironrdp_pdu::cursor::ReadCursor<'de>) -> PduResult<Self> {
        Ok(Self {
            version_major: src.read_u16(),
            version_minor: src.read_u16(),
            client_id: src.read_u32(),
        })
    }
}

mod codecs;
pub mod fast_path;
pub mod x224;

use bytes::{BufMut as _, Bytes, BytesMut};
use ironrdp_pdu::fast_path::{FastPathError, FastPathHeader};
use ironrdp_pdu::geometry::Rectangle;
use ironrdp_pdu::{DataHeader, PduHeader, PduParsing as _};

pub use self::x224::GfxHandler;
pub use crate::connection_sequence::{ConnectionSequenceResult, DesktopSize};
use crate::image::DecodedImage;
use crate::{utils, InputConfig, RdpError};

pub struct ActiveStageProcessor {
    x224_processor: x224::Processor,
    fast_path_processor: fast_path::Processor,
}

impl ActiveStageProcessor {
    pub fn new(
        config: InputConfig,
        graphics_handler: Option<Box<dyn GfxHandler + Send>>,
        connection_sequence_result: ConnectionSequenceResult,
    ) -> Self {
        let x224_processor = x224::Processor::new(
            utils::swap_hashmap_kv(connection_sequence_result.joined_static_channels),
            connection_sequence_result.initiator_id,
            connection_sequence_result.global_channel_id,
            config.graphics_config,
            graphics_handler,
        );

        let fast_path_processor = fast_path::ProcessorBuilder {
            global_channel_id: connection_sequence_result.global_channel_id,
            initiator_id: connection_sequence_result.initiator_id,
        }
        .build();

        Self {
            x224_processor,
            fast_path_processor,
        }
    }

    // TODO: async version?
    /// Sends a PDU on the dynamic channel. The upper layers are responsible for encoding the PDU and converting them to message
    pub fn send_dynamic(
        &mut self,
        stream: impl std::io::Write,
        channel_name: &str,
        message: Bytes,
    ) -> Result<(), RdpError> {
        self.x224_processor.send_dynamic(stream, channel_name, message)
    }

    // TODO: async version?
    /// Send a pdu on the static global channel. Typically used to send input events
    pub fn send_static(&self, stream: impl std::io::Write, message: ironrdp_pdu::ShareDataPdu) -> Result<(), RdpError> {
        self.x224_processor.send_static(stream, message)
    }

    pub fn process(&mut self, image: &mut DecodedImage, frame: Bytes) -> Result<Vec<ActiveStageOutput>, RdpError> {
        match process_header(&frame) {
            Ok(pdu_header) => match pdu_header {
                PduHeader::X224(_header) => self.process_x224_frame(_header, frame),
                PduHeader::FastPath(header) => self.process_fast_path_frame(header, frame, image),
            },
            Err(RdpError::FastPath(FastPathError::NullLength { bytes_read: _ })) => {
                warn!("Received null-length Fast-Path packet, dropping it");
                Ok(vec![])
            }
            Err(e) => Err(e),
        }
    }

    fn process_x224_frame(&mut self, _header: DataHeader, frame: Bytes) -> Result<Vec<ActiveStageOutput>, RdpError> {
        match self.x224_processor.process(frame) {
            Ok(output) => Ok(vec![ActiveStageOutput::ResponseFrame(output)]),
            Err(RdpError::UnexpectedDisconnection(message)) => {
                warn!("User-Initiated disconnection on Server: {}", message);
                Ok(vec![ActiveStageOutput::Terminate])
            }
            Err(RdpError::UnexpectedChannel(channel_id)) => {
                warn!("Got message on a channel with {} ID", channel_id);
                Ok(vec![ActiveStageOutput::Terminate])
            }
            Err(err) => Err(err),
        }
    }

    fn process_fast_path_frame(
        &mut self,
        header: FastPathHeader,
        frame: Bytes,
        image: &mut DecodedImage,
    ) -> Result<Vec<ActiveStageOutput>, RdpError> {
        let mut output_writer = BytesMut::new().writer();
        let graphics_update_region =
            self.fast_path_processor
                .process(image, &header, &frame[header.buffer_length()..], &mut output_writer)?;
        let output = output_writer.into_inner();

        let mut stage_outputs = Vec::new();

        if !output.is_empty() {
            stage_outputs.push(ActiveStageOutput::ResponseFrame(output));
        }

        if let Some(update_region) = graphics_update_region {
            stage_outputs.push(ActiveStageOutput::GraphicsUpdate(update_region));
        }

        Ok(stage_outputs)
    }
}

pub fn process_header(frame: &Bytes) -> Result<PduHeader, RdpError> {
    PduHeader::from_buffer(&frame[..]).map_err(RdpError::from)
}

pub enum ActiveStageOutput {
    ResponseFrame(BytesMut),
    GraphicsUpdate(Rectangle),
    Terminate,
}

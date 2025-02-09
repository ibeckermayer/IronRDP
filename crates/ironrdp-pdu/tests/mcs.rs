use ironrdp_pdu::mcs::*;
use ironrdp_pdu::{Error, PduParsing as _};
use ironrdp_pdu_samples::mcs::*;
use ironrdp_pdu_samples::mcs_encode_decode_test;

fn mcs_decode<'de, T: McsPdu<'de>>(src: &'de [u8]) -> ironrdp_pdu::Result<T> {
    let mut cursor = ironrdp_pdu::cursor::ReadCursor::new(src);
    T::mcs_body_decode(&mut cursor, src.len())
}

#[test]
fn invalid_domain_mcspdu() {
    let e = mcs_decode::<McsMessage>(&[0x48, 0x00, 0x00, 0x00, 0x70, 0x00, 0x01, 0x03, 0xEB, 0x70, 0x14])
        .err()
        .unwrap();

    if let Error::InvalidMessage { name, field, reason } = e {
        assert_eq!(name, "McsMessage");
        assert_eq!(field, "domain-mcspdu");
        assert_eq!(reason, "unexpected application tag for CHOICE");
    } else {
        panic!("unexpected error: {e}");
    }
}

mcs_encode_decode_test! {
    erect_domain_request: ERECT_DOMAIN_PDU, ERECT_DOMAIN_PDU_BUFFER;
    attach_user_request: ATTACH_USER_REQUEST_PDU, ATTACH_USER_REQUEST_PDU_BUFFER;
    attach_user_confirm: ATTACH_USER_CONFIRM_PDU, ATTACH_USER_CONFIRM_PDU_BUFFER;
    channel_join_request: CHANNEL_JOIN_REQUEST_PDU, CHANNEL_JOIN_REQUEST_PDU_BUFFER;
    channel_join_confirm: CHANNEL_JOIN_CONFIRM_PDU, CHANNEL_JOIN_CONFIRM_PDU_BUFFER;
    send_data_request: SEND_DATA_REQUEST_PDU, SEND_DATA_REQUEST_PDU_BUFFER;
    send_data_indication: SEND_DATA_INDICATION_PDU, SEND_DATA_INDICATION_PDU_BUFFER;
    disconnect_ultimatum: DISCONNECT_PROVIDER_ULTIMATUM_PDU, DISCONNECT_PROVIDER_ULTIMATUM_PDU_BUFFER;
}

#[test]
fn from_buffer_correct_parses_connect_initial() {
    let blocks = ConnectInitial::from_buffer(CONNECT_INITIAL_BUFFER.as_slice()).unwrap();
    assert_eq!(blocks, *CONNECT_INITIAL);
}

#[test]
fn to_buffer_correct_serializes_connect_initial() {
    let mut buf = Vec::new();
    CONNECT_INITIAL.to_buffer(&mut buf).unwrap();
    assert_eq!(buf, CONNECT_INITIAL_BUFFER);
}

#[test]
fn buffer_length_is_correct_for_connect_initial() {
    let len = CONNECT_INITIAL.buffer_length();
    assert_eq!(len, CONNECT_INITIAL_BUFFER.len());
}

#[test]
fn from_buffer_correct_parses_connect_response() {
    let blocks = ConnectResponse::from_buffer(CONNECT_RESPONSE_BUFFER.as_slice()).unwrap();
    assert_eq!(blocks, *CONNECT_RESPONSE);
}

#[test]
fn to_buffer_correct_serializes_connect_response() {
    let mut buf = Vec::new();
    CONNECT_RESPONSE.to_buffer(&mut buf).unwrap();
    assert_eq!(buf, CONNECT_RESPONSE_BUFFER);
}

#[test]
fn buffer_length_is_correct_for_connect_response() {
    let len = CONNECT_RESPONSE.buffer_length();
    assert_eq!(len, CONNECT_RESPONSE_BUFFER.len());
}

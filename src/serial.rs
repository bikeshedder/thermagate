fn crc(buf: &[u8]) -> u8 {
    !buf.iter().fold(0u8, |s, &b| { s.wrapping_add(b) })
}

pub fn reg_query(reg_id: u8) -> Vec<u8> {
    let mut msg = vec![0x02, reg_id];
    msg.push(crc(&msg));
    msg
}

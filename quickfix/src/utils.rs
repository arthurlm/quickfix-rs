pub fn read_buffer_to_string(buffer: &[u8]) -> String {
    let null_index = buffer.iter().position(|x| *x == 0).unwrap_or(buffer.len());
    String::from_utf8_lossy(&buffer[..null_index]).to_string()
}

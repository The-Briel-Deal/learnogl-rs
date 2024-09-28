pub fn add_null_term(str: &[u8]) -> Vec<u8> {
    let mut str = Vec::from(str);
    str.push(b'\0');
    str
}

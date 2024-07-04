use md5;

//http://www.cse.yorku.ca/~oz/hash.html
#[allow(dead_code)]
pub fn djb2_u16(text: String) -> u16 {
    let mut result: u16 = 5318;
    for c in text.chars() {
        let to_add = c as u16;
        result = result.wrapping_mul(33).wrapping_add(to_add);
    }

    return result;
}

pub fn md5_u16(text: String) -> u16 {
    let digest = md5::compute(text);
    
    return u16::from_le_bytes([digest[0], digest[1]]);
}

// very dumb hashing
#[allow(dead_code)]
pub fn simple(text: String) -> u16 {
    let mut result: u16 = 0;
    for c in text.chars() {
        result = result.wrapping_add(c as u16);
    }

    return result;
}

pub fn print(hash: u16) {
    println!("Hash: {:016b}", hash);
}

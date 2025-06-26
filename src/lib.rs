pub fn url_decode(s: &str) -> String {
    let mut result = Vec::new();
    let mut chars = s.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '%' {
            let hi = chars.next();
            let lo = chars.next();
            if let (Some(hi), Some(lo)) = (hi, lo) {
                let hex = [hi, lo].iter().collect::<String>();
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    result.push(byte);
                    continue;
                }
            }
            // If decoding failed, add the original % and characters
            result.push(b'%');
            if let Some(hi) = hi { 
                // Convert character to UTF-8 bytes
                let mut bytes = [0; 4];
                let utf8_bytes = hi.encode_utf8(&mut bytes);
                result.extend_from_slice(utf8_bytes.as_bytes());
            }
            if let Some(lo) = lo { 
                // Convert character to UTF-8 bytes
                let mut bytes = [0; 4];
                let utf8_bytes = lo.encode_utf8(&mut bytes);
                result.extend_from_slice(utf8_bytes.as_bytes());
            }
        } else {
            // Convert character to UTF-8 bytes
            let mut bytes = [0; 4];
            let utf8_bytes = ch.encode_utf8(&mut bytes);
            result.extend_from_slice(utf8_bytes.as_bytes());
        }
    }
    
    // Convert the byte vector back to a UTF-8 string
    String::from_utf8_lossy(&result).into_owned()
}

#[cfg(test)]
mod tests; 
const DATE_PREFIX: &[u8] = b"D:";
const DATE_LEN: usize = 14;
const FIXED_DATE: &[u8] = b"20000101000000";
const ID_PREFIX: &[u8] = b"/ID (";
const NODE_PREFIX: &[u8] = b"node";
const NODE_DIGITS: usize = 8;
const FIXED_DIGIT: u8 = b'0';

pub fn normalize(bytes: &[u8]) -> Vec<u8> {
    let mut out = bytes.to_vec();
    pin_dates(&mut out);
    pin_trailer_ids(&mut out);
    pin_node_ids(&mut out);
    out
}

fn pin_dates(out: &mut [u8]) {
    let mut index = 0;
    while index + DATE_PREFIX.len() + DATE_LEN <= out.len() {
        let start = index + DATE_PREFIX.len();
        if out[index..].starts_with(DATE_PREFIX)
            && out[start..start + DATE_LEN].iter().all(u8::is_ascii_digit)
        {
            out[start..start + DATE_LEN].copy_from_slice(FIXED_DATE);
            index = start + DATE_LEN;
        } else {
            index += 1;
        }
    }
}

fn pin_trailer_ids(out: &mut [u8]) {
    let mut index = 0;
    while index + ID_PREFIX.len() <= out.len() {
        if out[index..].starts_with(ID_PREFIX) {
            let start = index + ID_PREFIX.len();
            if let Some(offset) = out[start..].iter().position(|byte| *byte == b')') {
                let end = start + offset;
                out[start..end].fill(FIXED_DIGIT);
                index = end;
                continue;
            }
        }
        index += 1;
    }
}

fn pin_node_ids(out: &mut [u8]) {
    let mut index = 0;
    while index + NODE_PREFIX.len() + NODE_DIGITS <= out.len() {
        let start = index + NODE_PREFIX.len();
        if out[index..].starts_with(NODE_PREFIX)
            && out[start..start + NODE_DIGITS].iter().all(u8::is_ascii_digit)
        {
            out[start..start + NODE_DIGITS].fill(FIXED_DIGIT);
            index = start + NODE_DIGITS;
        } else {
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::normalize;

    #[test]
    fn pins_dates_ids_and_node_counters_without_changing_length() {
        let raw = b"/CreationDate (D:20261014185020+00'00') /ID (node00000043) /Limits [(node00000089)]";
        let pinned = normalize(raw);
        assert_eq!(pinned.len(), raw.len());
        let text = String::from_utf8(pinned).expect("ascii");
        assert!(text.contains("D:20000101000000"));
        assert!(text.contains("/ID (000000000000)"));
        assert!(text.contains("node00000000"));
        assert!(!text.contains("node00000089"));
    }
}

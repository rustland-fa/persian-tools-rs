pub mod landline;
pub mod mobile;

static NUMBER_PREFIX_STR: [&str; 4] = ["+98", "0", "98", "0098"];

fn get_num_skip_str(text: &str) -> usize {
    for prefix in NUMBER_PREFIX_STR {
        if text.starts_with(prefix) {
            return prefix.len();
        }
    }
    0
}

fn validate_num(num: u64) -> Option<u64> {
    let len = num.checked_ilog10().unwrap_or(0) + 1;
    match len {
        12 => {
            if num / 10u64.pow(10) == 98 {
                Some(num % 10u64.pow(10))
            } else {
                None
            }
        }
        10 => Some(num),
        _ => None,
    }
}

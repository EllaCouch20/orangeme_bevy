use super::Error;

use super::pub_structs::KeyPress;

pub fn update_amount(amount: String, key: KeyPress) -> Result<(String, bool, u8), Error> {
    let is_zero = || amount == "0";
    let zero = "0".to_string();

    let (updated_amount, valid_input) = match key {
        KeyPress::Reset => ("0".to_string(), true),
        KeyPress::Backspace => {
            if is_zero() {
                (zero, false)
            } else if amount.len() == 1 {
                (zero, true)
            } else {
                (amount[..amount.len() - 1].to_string(), true)
            }
        },
        KeyPress::Decimal => {
            if !amount.contains('.') && amount.len() <= 7 {
                (format!("{}{}", amount, "."), true)
            } else {
                (amount.clone(), false)
            }
        },
        input => {
            let input = input as i64;
            if is_zero() {
                (input.to_string(), true)
            } else if amount.contains('.') {
                let split: Vec<&str> = amount.split('.').collect();
                if amount.len() < 11 && split[1].len() < 2 {
                    (format!("{}{}", amount, input), true)
                } else {
                    (amount.clone(), false)
                }
            } else if amount.len() < 10 {
                (format!("{}{}", amount, input), true)
            } else {
                (amount.clone(), false)
            }
        }
    };

    let needed_placeholders = if updated_amount.contains('.') {
        let split: Vec<&str> = updated_amount.split('.').collect();
        2-split.get(1).unwrap_or(&"").len() as u8
    } else {0};
    Ok((updated_amount, valid_input, needed_placeholders))
}

use common::{FromSigned, PartAnswer};

const LAYER_TOTALS_SIZE: usize = 64;

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let mut idx = 0;
    let mut start_idx = 0;
    let mut reading_num = false;

    let mut obj_level = 0;
    let mut obj_valid = true;
    let mut invalid_level = 0;
    let mut layer_totals = [0; LAYER_TOTALS_SIZE];

    let mut a = 0;

    while idx < input.len() {
        if reading_num {
            if !input[idx].is_ascii_digit() {
                let num = i32::from_signed(&input[start_idx..idx]);
                a += num;
                if obj_valid {
                    layer_totals[obj_level] += num;
                }
                reading_num = false;
            }
        } else if input[idx] == b'-' || input[idx].is_ascii_digit() {
            reading_num = true;
            start_idx = idx;
        }

        if input[idx] == b'{' {
            obj_level += 1;
        } else if input[idx] == b'}' {
            obj_level -= 1;

            if obj_valid {
                layer_totals[obj_level] += layer_totals[obj_level + 1];
                layer_totals[obj_level + 1] = 0;
            } else if obj_level < invalid_level {
                obj_valid = true;
            }
        } else if obj_valid
            && let Some(s) = input.get(idx..idx + 6)
            && s == b":\"red\""
        {
            obj_valid = false;
            invalid_level = obj_level;
            layer_totals[obj_level] = 0;
        }

        idx += 1;
    }

    [a.into(), layer_totals[0].into()]
}

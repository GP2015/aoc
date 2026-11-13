use crate::PartAnswer;

fn next_dims(input_idx: &mut usize, input: &[u8]) -> [u32; 3] {
    let mut dims = [0; 3];

    for dim in &mut dims {
        #[allow(clippy::indexing_slicing)]
        let d0 = input[*input_idx];
        let d1_opt = input.get(*input_idx + 1).copied();

        *dim = u32::from(d0 - b'0');
        *input_idx += 2;

        if let Some(d1) = d1_opt
            && d1.is_ascii_digit()
        {
            *dim *= 10;
            *dim += u32::from(d1 - b'0');
            *input_idx += 1;
        }
    }

    if dims[0] > dims[1] {
        dims.swap(0, 1);
    }

    if dims[1] > dims[2] {
        dims.swap(1, 2);
    }

    dims
}

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let mut a = 0;
    let mut b = 0;

    let mut input_idx = 0;

    while input_idx < input.len() {
        let dims = next_dims(&mut input_idx, input);
        a += 3 * dims[0] * dims[1] + 2 * dims[2] * (dims[0] + dims[1]);
        b += 2 * (dims[0] + dims[1]) + (dims[0] * dims[1] * dims[2]);
    }

    [a.into(), b.into()]
}

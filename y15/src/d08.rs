use common::PartAnswer;

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    input
        .split(|&c| c == b'\n')
        .map(|line| &line[1..line.len() - 1])
        .map(|line| {
            let mut a = 2;
            let mut idx = 0;
            while idx < line.len() {
                if line[idx] == b'\\' {
                    if line[idx + 1] == b'x' {
                        a += 3;
                        idx += 4;
                    } else {
                        a += 1;
                        idx += 2;
                    }
                } else {
                    idx += 1;
                }
            }

            let b = 4 + line.iter().filter(|&&c| c == b'\\' || c == b'\"').count();

            [a, b]
        })
        .fold([0, 0], |mut acc, item| {
            acc[0] += item[0];
            acc[1] += item[1];
            acc
        })
        .map(Into::into)
}

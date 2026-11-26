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
        .fold([0, 0], |[a, b], [i0, i1]| [a + i0, b + i1])
        .map(Into::into)
}

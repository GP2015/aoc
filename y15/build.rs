use std::{env, fs::File, io::Write, path::Path};

const MOST_RECENT_DAY: usize = 1;

pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gen.rs");
    let mut f = File::create(dest_path).unwrap();

    writeln!(f, "fn run(day: u8, part: char) {{ match day {{").unwrap();

    for i in 1..=MOST_RECENT_DAY {
        writeln!(f, "{i} => d{i:02}::Day::part(part),").unwrap();
    }

    writeln!(f, " _ => unreachable!(), }} }}").unwrap();
}

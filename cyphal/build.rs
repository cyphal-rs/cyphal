use std::{
    env::var,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

pub const CASTAGNOLI_POLYNOMIAL: u32 = 0x82f63b78;

fn main() {
    let out_dir = var("OUT_DIR").expect("Unable to retrieve the OUT_DIR environment variable");
    let path = Path::new(&out_dir).join("crc_table_data.rs");

    let mut table: [[u32; 256]; 8] = [[0u32; 256]; 8];

    for n in 0..256_u32 {
        let mut crc = n;

        for _ in 0..8 {
            if crc % 2 == 0 {
                crc /= 2;
            } else {
                crc /= 2;
                crc ^= CASTAGNOLI_POLYNOMIAL;
            }
        }

        table[0][n as usize] = crc;
    }

    for n in 0..256 {
        let mut crc = table[0][n as usize];
        for k in 1..8 {
            crc = table[0][(crc as u8) as usize] ^ (crc >> 8);
            table[k as usize][n as usize] = crc;
        }
    }

    let mut file = {
        let file = File::create(path).expect("Unable to create the crc_table_data.rs file");
        BufWriter::new(file)
    };

    write!(file, "pub(crate) const CRC_TABLE: CrcTable = CrcTable([\n")
        .expect("Unable to write crc table");

    for row in table {
        write!(file, "[").expect("Unable to write crc table");

        for (i, element) in row.iter().enumerate() {
            write!(file, "{}", element.to_string()).expect("Unable to write crc table");

            if i == row.len() - 1 {
                write!(file, "\n").expect("Unable to write crc table");
            } else if (i + 1) % 6 == 0 {
                write!(file, ",\n").expect("Unable to write crc table");
            } else {
                write!(file, ", ").expect("Unable to write crc table");
            }
        }
        write!(file, "],\n").expect("Unable to write crc table");
    }

    write!(file, "]);\n").expect("Unable to write crc table");

    println!("cargo:rerun-if-changed=build.rs");
}

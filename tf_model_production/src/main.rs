use std::io::prelude::*;
use std::fs::File;
use byteorder::{BigEndian,ReadBytesExt};
//use tensorflow;

const TEST_IMG_FILE: &str = "data/t10k-images.idx3-ubyte";
const TEST_LBL_FILE: &str = "data/t10k-labels.idx1-ubyte";


fn main() -> Result<(), std::io::Error> {
    let mut contents: Vec<u8> = Vec::new();
    let mut file = {
        let mut fh = File::open(TEST_IMG_FILE)?;
        let _ = fh.read_to_end(&mut contents)?;
        // the read_u32() method cannot use a Vec directly;
        // it requires a slice
        &contents[..]
    };

    let magic_number = file.read_u32::<BigEndian>()?;
    let length = file.read_u32::<BigEndian>()?;
    let rows = file.read_u32::<BigEndian>()?;
    let cols = file.read_u32::<BigEndian>()?;

    // TODO: this just converts the rest of the file to one long vector;
    // we need to use `length`, `rows`, and `cols` to split further
    file.to_vec();

    println!("{}", file.len());
    println!("We're oooooookay!");
    Ok(())
}

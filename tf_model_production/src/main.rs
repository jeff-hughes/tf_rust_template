use std::io::prelude::*;
use std::fs::File;
use byteorder::{BigEndian,ReadBytesExt};
//use tensorflow;

const TEST_IMG_FILE: &str = "data/t10k-images.idx3-ubyte";
const TEST_LBL_FILE: &str = "data/t10k-labels.idx1-ubyte";
const IMG_MAGIC_NUMBER: u32 = 0x00000803;
const LBL_MAGIC_NUMBER: u32 = 0x00000801;

const NUM_IMG: usize = 50;


struct Mnist {
    tensor: Vec<u8>,
    label: u8,
}


fn process_mnist(filename: &str) -> (Vec<u8>, usize, Option<(usize, usize)>) {
    let mut contents: Vec<u8> = Vec::new();
    let mut file = {
        let mut fh = File::open(filename).unwrap();
        let _ = fh.read_to_end(&mut contents).unwrap();
        // the read_u32() method cannot use a Vec directly;
        // it requires a slice
        &contents[..]
    };

    let magic_number = file.read_u32::<BigEndian>().unwrap();
    let length = file.read_u32::<BigEndian>().unwrap() as usize;

    let sizes = match magic_number {
        LBL_MAGIC_NUMBER => None,
        IMG_MAGIC_NUMBER => Some((
            file.read_u32::<BigEndian>().unwrap() as usize,
            file.read_u32::<BigEndian>().unwrap() as usize,
        )),
        _ => panic!(),
    };

    (file.to_vec(), length, sizes)
}

fn main() -> Result<(), std::io::Error> {
    let (img, _img_length, img_sizes) = process_mnist(TEST_IMG_FILE);
    let (lbl, _lbl_length, _lbl_sizes) = process_mnist(TEST_LBL_FILE);

    let rc = img_sizes.unwrap();
    let mut mnist_data: Vec<Mnist> = Vec::new();
    for x in 0..NUM_IMG {
        let vec_ln = rc.0 * rc.1;
        mnist_data.push(Mnist {
            tensor: img[x..((x+1)*vec_ln)].to_vec(),
            label: lbl[x],
        });
    }

    println!("{}", mnist_data.len());




    Ok(())
}

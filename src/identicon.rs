mod hash;

use std::io::{self, BufRead};

struct Image<'a>{
    data: &'a [bool],
    rgb: &'a [u8]
}
/*
    assuming text is converted to the 5x5, and knowing that images is mirrored by x. 2x5 + 5 = 15, so we need 15 bits.
    By using u16 we can fit all 15 bits, and we left with one extra. We will be using all 16 bit value for the RBG 565  

                        abcba
                        defed
    abcdefjighklmnop -> ghihg   
                        jklkj
                        mnonm

    color: RBG 565 
            R: abcde G: efjigk B: lmnop
*/

pub fn run() {
    let stdin = io::stdin();
    loop {
        println!("Please input the text (or type 'exit' to quit):");
        let mut input_text = String::new();
        stdin.lock().read_line(&mut input_text).expect("Error reading value");
        input_text = input_text.trim().to_string();
        if input_text.to_lowercase() == "exit" {
            break;
        }
        create_identicon(input_text);
    }
}


fn create_identicon(text: String) {
    let hash = hash_text(text);
    hash::print(hash);
    visualize(hash);
}

fn hash_text(text: String) -> u16 {
    return hash::md5_u16(text);
}

fn visualize(hash: u16) {
    // if hash is zero nothing to draw
    if hash == 0 {
        return;
    }

    let mut working_hash = hash;
    let first_bit: u16 = 0b1<<15; // basically 0b1000000000000000

    // shift hash until first bit is non zero
    loop {
        let res = working_hash & first_bit;
        if res == first_bit {
            break;
        }
        working_hash = working_hash<<1;
    }

    let mut img_data: [bool; 25] = [false; 25]; 
    for i in 0..15 {
        let res = working_hash & first_bit;
        let fill = res == first_bit;
        working_hash = working_hash<<1;

        let row1 = i%3;
        let row2 = 4-row1;
        let col = i/3;

        img_data[col*5 + row1] = fill;
        if row2 != row1 {
            // mirrored side
            img_data[col*5 + row2] = fill;
        }
    }
    working_hash = hash;
    let mut rgb: [u8; 3] = [0; 3];
    rgb[2] = (working_hash as u8) & 0b11111;
    working_hash = working_hash >> 5;
    rgb[1] = (working_hash as u8) & 0b111111;
    working_hash = working_hash >> 6;
    rgb[0] = (working_hash as u8) & 0b11111;
    println!("Image (R {:05b} G {:06b} B: {:05b}):",
        rgb[0],
        rgb[1],
        rgb[2]
    );
    
    for i in 0..5 {
        for j in 0..5 {
            print!("{} ", if img_data[i*5 + j] {"1"} else {"0"});
        }
        println!();
    }
    println!();
    let img = Image {
        data: &img_data,
        rgb: &rgb
    };
    generate_image(&img);
}

use std::ops::Add;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn generate_image(img: &Image) {
    let path = Path::new("image.ppm");
    let display = path.display();
    
    let mut file = match File::create(&path) {
        Err(why) => panic!("can not create {}: {}", display, why),
        Ok(file) => file,
    };

    let output_image_size = 500;
    let header = format!("P3\n{} {}\n255\n", output_image_size, output_image_size);
    file.write(header.as_bytes()).unwrap();

    fn to_color(value: u8, size: u8)-> u8 {
        if size == 0 {
            panic!("Size can't be zero");
        }
        let max:u8 = (1 << size) - 1;
        return (((value as f32)/(max as f32)) * (255 as f32)).round() as u8
    }

    let mut rbg: [u8; 3] = [0; 3];
    rbg[0] = to_color(img.rgb[0], 5);
    rbg[1] = to_color(img.rgb[1], 6);
    rbg[2] = to_color(img.rgb[2], 5);

    let color = format!("{} {} {}", rbg[0], rbg[1], rbg[2]);
    println!("color : {}", color);
    let white_color = "255 255 255".to_string();
    let mut file_content: String = String::new();

    let scale = output_image_size/5;
    for row in 0..output_image_size {
        let source_row = row/scale;
        for col in 0..output_image_size {
            let source_col = col/scale;
            let source_idx = (source_row*5)+source_col;
            if img.data[source_idx] == false {
                file_content = file_content + white_color.as_str();
            } else {
                file_content = file_content + color.as_str();
            }
            file_content = file_content + "\n";
        }
    }
    file.write(file_content.as_bytes()).unwrap();

}




pub fn run() {
    let nx = 3;
    let ny = 3;

}

fn hash_image(nx: u32, nx: u32) {
    let first_digit = (nx-1) + (ny - 1)*9;
    let max:f32 = 1.0;
    let max_comp = (max+1)/166;
    let avg_rgb: [u8; 3] = [0;3];
    let avg_color: u32 = 0;
    for i in 0..3 {
        avg_color = avg_rgb[i];
        avg_color << ((2-i)*8);
    }
    
    // https://dev.to/marycheung021213/understanding-dct-and-quantization-in-jpeg-compression-1col

}
use std::fs;
use std::str::FromStr;
use std::ops::Index;

pub fn soln_a(file: &str) -> Result<u32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let alg = &file_str[0..512];
    let img_str = &file_str[514..];
    // println!("alg: {}", alg);
    // println!("img_str: {}", img_str);
    let image: Image = img_str.parse().unwrap();

    let key = ImageEnhancer::parse_key(alg);
    let inf_bit_flips = key[0] == 1;
    let mut enhancer = ImageEnhancer {
        image: Image::pad_image(&image, 2),
        key,
        inf_bit_flips,
        inf_bit: 0
    };
    println!("{:?}", enhancer);
    enhancer.image.display();

    enhancer.enhance();
    enhancer.image.display();
    enhancer.enhance();
    enhancer.image.display();
    println!("nrows: {}, ncols: {}", enhancer.image.nrows, enhancer.image.ncols);
    Ok(enhancer.image.count_lit_pixels())
}

pub fn soln_b(file: &str) -> Result<u32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let alg = &file_str[0..512];
    let img_str = &file_str[514..];
    // println!("alg: {}", alg);
    // println!("img_str: {}", img_str);
    let image: Image = img_str.parse().unwrap();

    let key = ImageEnhancer::parse_key(alg);
    let inf_bit_flips = key[0] == 1;
    let mut enhancer = ImageEnhancer {
        image: Image::pad_image(&image, 12),
        key,
        inf_bit_flips,
        inf_bit: 0
    };
    for _ in 0..50 {
        enhancer.enhance();
    }
    println!("nrows: {}, ncols: {}", enhancer.image.nrows, enhancer.image.ncols);
    Ok(enhancer.image.count_lit_pixels())
}

#[derive(Debug)]
struct ImageEnhancer {
    image: Image,
    inf_bit: u8,
    inf_bit_flips: bool,
    key: Vec<u8>
}

impl ImageEnhancer {
    fn parse_key(s: &str) -> Vec<u8> {
        s.chars().map(|c| if c == '#' { 1 } else { 0 }).collect()
    }
    fn enhance(&mut self) {
        let mut new_img = self.image.clone();
        for i in 0..new_img.nrows {
            for j in 0..new_img.ncols {
                let key_idx: usize = self.image.get_square(i,j, self.inf_bit);
                let bit_val = self.key[key_idx];
                new_img.update_bit(i,j, bit_val);
            }
        }
        self.image = new_img;
        if self.inf_bit_flips {
            // flip inf_bit
            if self.inf_bit == 0 {
                self.inf_bit = 1;
            } else {
                self.inf_bit = 0;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Image {
    bytes: Box<[u8]>,
    nrows: usize,
    ncols: usize,
}

impl Image {
    fn get_1d(&self, i: usize) -> u8 {
        // println!("byte: {:b}", self.bytes[i / 8]);
        return (self.bytes[i / 8] >> (7 - (i % 8))) & 1
    }
    fn get_unchecked(&self, i: usize, j: usize) -> u8 {
        return self.get_1d(i*self.ncols + j);
    }

    fn get(&self, i: usize, j: usize) -> Option<u8> {
        if i >= self.nrows || j >= self.ncols {
            return None;
        }
        return Some(self.get_1d(i*self.ncols + j));
    }
    fn display(&self) {
        let mut i = 0;
        for i in 0..self.nrows {
            for j in 0..self.ncols {
                if self.get(i,j).unwrap() == 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
        println!("")
    }

    fn get_square(&self, i: usize, j: usize, default: u8) -> usize {
        let mut idx: usize = 0; // no more than 9 bits
        let mut bits_read = vec![];
        for di in 0..=2 {
            for dj in 0..=2 {
                if i + di < 1 || j + dj < 1 {
                    let bit = default as usize;
                    bits_read.push(bit);
                    idx = idx << 1 | (1 & bit);
                } else {
                    let bit = self.get(i + di - 1, j + dj - 1).unwrap_or(default) as usize;
                    bits_read.push(bit);
                    idx = idx << 1 | (1 & bit);
                }
            }
        }
        if idx != 0 {
            // println!("idx: {} \t{:b}", idx, idx);
            let chars: String = bits_read.iter().map(|d| if *d == 1 { '#' } else { '.' }).collect();
            // println!("bits read: {:?}", chars);
        }
        return idx;
    }

    fn update_bit(&mut self, i: usize, j: usize, val: u8) {
        let idx_1d = i * self.ncols + j;
        let byte: u8 = self.bytes[idx_1d / 8];
        let mask: u8 = 1 << (7 - idx_1d % 8); 
        self.bytes[idx_1d / 8] = (byte & !mask) | ((val & 1) << (7 - idx_1d % 8));
    }

    fn update_byte(&mut self, byte: u8, idx_1d: usize) {
        self.bytes[idx_1d] = byte;
    }
    
    fn count_lit_pixels(&self) -> u32 {
        let mut sum: u32 = 0;
        for byte in self.bytes.iter() {
            for i in 0..8 {
                sum += ((byte >> i) & 1) as u32
            }
        }
        sum
    }
    
    fn pad_image(image: &Image, n: usize) -> Image {
        // n is in bytes (i.e. 8 rows and cols at a time)
        //          XXXXX
        // BBB      XBBBX
        // BBB  ->  XBBBX
        // BBB      XBBBX
        //          XXXXX
        let ncols = image.ncols + n * 16;
        let nrows = image.nrows + n * 16;

        let bytes = (nrows * ncols/8);
        //println!("bytes: {}", bytes);
        let mut bytes = vec![0; bytes];
        let mut padded_image = Image {
            bytes: bytes.into_boxed_slice(),
            nrows,
            ncols,
        };
        let row_offset = n * 8;
        let col_offset = n;
        for i in 0..image.nrows {
            for j in 0..(image.ncols / 8) {
                padded_image.bytes[(i + row_offset) * ncols / 8 + (j + col_offset)] = image.bytes[i * image.ncols / 8 + j];
            }
        }
        padded_image
    }
}

impl FromStr for Image {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let nrows = lines.len();
        let min_cols = lines[0].len();
        let bytes_per_row = (min_cols - 1) / 8 + 1; // pad each row so the number of columns is a multiple of 8
        let ncols = bytes_per_row * 8;
        let mut bytes: Vec<u8> = Vec::with_capacity(nrows * bytes_per_row);
        let mut i = 0;
        let mut byte: u8 = 0;
        for l in lines.into_iter() {
            let mut i = 0;
            loop {
                let end = if i + 8 < ncols {i + 8} else {min_cols};
                let byte = &l[i..end];
                let (remaining, byte): (usize, u8) = byte.chars().fold((8, 0), |(n, byte), c| (n - 1, byte << 1 | (c == '#') as u8));
                if remaining > 0 {
                    bytes.push(byte << remaining);
                    break;
                }
                bytes.push(byte);
                i += 8;
            }
            
        }
        Ok(Image {
            bytes: bytes.into_boxed_slice(),
            nrows,
            ncols,
        })
    }
}

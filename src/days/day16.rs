use std::{fs};

pub fn soln_a(file: &str) -> Result<u64, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let bits: Box<Vec<u8>> = Box::new(file_str.trim().chars().flat_map(|c| {
        let d = c.to_digit(16).unwrap() as u8;
        return (0..=3).rev().map(move |n| d >> n & 1);
    }).collect());
    let mut r = BitReader {
        bits,
        curr: 0
    };
    let packet = r.parse_packet();
    Ok(packet.sum_version_nums())
}

pub fn soln_b(file: &str) -> Result<u64, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let bits: Box<Vec<u8>> = Box::new(file_str.trim().chars().flat_map(|c| {
        let d = c.to_digit(16).unwrap() as u8;
        return (0..=3).rev().map(move |n| d >> n & 1);
    }).collect());
    let mut r = BitReader {
        bits,
        curr: 0
    };
    let packet = r.parse_packet();
    Ok(packet.eval())
}



struct BitReader {
    bits: Box<Vec<u8>>,
    curr: usize,
}

impl BitReader {
    fn parse_num(&mut self, n: usize) -> u64 {
        let mut val = 0;
        for b in self.bits[self.curr..self.curr+n].iter() {
            val = val << 1 | (*b as u64);
        }
        self.curr += n;
        val
    }
    fn parse_bool(&mut self) -> bool {
        self.curr += 1;
        return self.bits[self.curr - 1] == 1;
    }
    
    fn parse_packet(&mut self) -> Packet {
        let version = self.parse_num(3) as u8;
        let type_id = self.parse_num(3) as u8;
        if type_id == 4 { // literal packet
            let value = self.parse_literal_val();
            return Packet::LiteralPacket {
                version,
                value
            }
        } else { // operator packet
            let length_type = self.parse_bool();
            let mut subpackets = vec![];
            if length_type { // number sub-packets
                let subpackets_n = self.parse_num(11);
                for _ in 0..subpackets_n {
                    subpackets.push(self.parse_packet());
                }
            } else {
                let subpacket_n_bits = self.parse_num(15) as usize;
                let start_i = self.curr;
                while self.curr - start_i < subpacket_n_bits {
                    subpackets.push(self.parse_packet());
                }
            }
            return Packet::OperatorPacket {
                version,
                type_id,
                subpackets
            }
        }
    }
    
    fn parse_literal_val(&mut self) -> u64 {
        let is_last = !self.parse_bool();
        if is_last {
            return self.parse_num(4);
        } else {
            let start_i = self.curr;
            let high_order = self.parse_num(4);
            let low_order = self.parse_literal_val();
            let n_bits = (self.curr - start_i) / 5 * 4;
            return high_order << n_bits | low_order;
        }
    }
}


#[derive(Debug)]
enum Packet {
    LiteralPacket {version: u8, value: u64},
    OperatorPacket {version: u8, type_id: u8, subpackets: Vec<Packet>}
}

impl Packet {
    fn sum_version_nums(&self) -> u64 {
        match self {
            Packet::LiteralPacket{version, ..} => *version as u64,
            Packet::OperatorPacket{version, subpackets,..} => {
                let rest: u64 = subpackets.iter().map(|p| p.sum_version_nums()).sum();
                (*version as u64) + rest
            }
        }
    }
    fn eval(&self) -> u64 {
        match self {
            Packet::LiteralPacket{value, ..} => *value,
            Packet::OperatorPacket{type_id, subpackets,..} => {
                let sub_iter = subpackets.into_iter();
                match *type_id {
                    0 => sub_iter.map(|p| p.eval()).sum(), // sum
                    1 => sub_iter.fold(1, |acc, p| acc * p.eval()), // product
                    2 => sub_iter.map(|p| p.eval()).min().expect("empty subpacket array"), // min
                    3 => sub_iter.map(|p| p.eval()).max().expect("empty subpacket array"), // min
                    5 => if subpackets[0].eval() > subpackets[1].eval() { 1 } else { 0 }, // GT
                    6 => if subpackets[0].eval() < subpackets[1].eval() { 1 } else { 0 }, // LT
                    7 => if subpackets[0].eval() == subpackets[1].eval() { 1 } else { 0 }, // EQ
                    _ => panic!("invalid type id")
                }
            }
        }
    }
}

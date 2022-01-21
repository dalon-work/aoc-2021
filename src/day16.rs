// Inspired by https://github.com/mikefarquhar/advent_of_code_2021/blob/main/16/src/main.rs
//
// My first attempt was a huge mess. These parsing problems are hard for me.
// Although I did learn a few valuable lessons from the example I found.
//   * break it down into small pieces
//   * You can use structs in enums, which is quite handy.
//     I orignally tried to make the a Packet like this:
//       enum PacketType {
//          Literal,
//          Operator
//        }
//
//        struct Packet {
//          Header,
//          PacketType
//        }
//    and that was really messy.
//   * parser state should be stored in an actual struct, not 
//     passed around through the functions arguments
//   * Store just the total bit position and calculate the byte on the fly,
//     don't try to track the byte and the bit inside the byte. That
//     was messy. By tracking the total bit position, 
//     it greatly simplifies that parsing the operator packets of type 0.

use hex::FromHex;

struct Header {
    version: u64,
    packet_type: u64,
}

struct Literal {
    header: Header,
    value: u64,
}

struct Operator {
    header: Header,
    packets: Vec<Packet>
}

enum Packet {
    Literal(Literal),
    Operator(Operator),
}

impl Packet {
    fn add_version_numbers(&self) -> u64 {
        match self {
            Packet::Literal(literal) => {
                return literal.header.version;
            },
            Packet::Operator(operator) => {
                let mut v = operator.header.version;
                for p in &operator.packets {
                    v += p.add_version_numbers();
                }
                return v;
            }
        }
    }

    fn do_math(&self) -> u64 {
        match self {
            Packet::Literal(literal) => {
                return literal.value;
            },
            Packet::Operator(operator) => {
                match operator.header.packet_type {
                    0 => operator.packets.iter().map(|p| p.do_math()).sum(),
                    1 => operator.packets.iter().map(|p| p.do_math()).product(),
                    2 => operator.packets.iter().map(|p| p.do_math()).min().unwrap(),
                    3 => operator.packets.iter().map(|p| p.do_math()).max().unwrap(),
                    5 => (operator.packets[0].do_math() > operator.packets[1].do_math()) as u64,
                    6 => (operator.packets[0].do_math() < operator.packets[1].do_math()) as u64,
                    7 => (operator.packets[0].do_math() == operator.packets[1].do_math()) as u64,
                    _ => unreachable!(),
                }
            }
        }
    }
}

struct PacketParser {
    bytes: Vec<u8>,
    bit_pos: usize,
}

fn get_mask(count: usize) -> u64 {
    (1u64 << (count))-1  // sets the first count bits
}

impl PacketParser {
    fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            bit_pos: 0
        }
    }

    fn read_u64(&mut self, mut count: usize) -> u64 {
        let mut value: u64 = 0;
        while count > 0 {
            let cur_byte = self.bit_pos / 8;
            let cur_bit = self.bit_pos % 8;
            let byte = self.bytes[cur_byte] as u64;
            let avail = 8-cur_bit;
            if avail > count { // we will not finish consuming this byte
                value <<= count;
                let mask = get_mask(count);
                let shift = 8-cur_bit-count;
                value |= mask & ((byte as u64) >> shift);
                self.bit_pos += count;
                count -= count;
            } else { // we will consume the rest of this byte
                value <<= avail;
                let mask = get_mask(avail);
                value |= mask & byte;
                self.bit_pos += avail;
                count -= avail;
            }
        }
        return value
    }

    fn read_header(&mut self) -> Header {
        Header{
            version: self.read_u64(3),
            packet_type: self.read_u64(3),
        }
    }

    fn parse_packet(&mut self) -> Packet {
        let header = self.read_header();
        match header.packet_type {
            4 => self.read_literal(header),
            _ => self.read_operator(header),
        }
    }

    fn read_literal(&mut self, header: Header) -> Packet {
        let mut value: u64 = 0;
        loop {
            let has_next = self.read_u64(1);
            let new_bits = self.read_u64(4);
            value = (value << 4) | new_bits;
            if has_next == 0 {
                break;
            }
        }

        let literal = Literal { header, value };
        return Packet::Literal(literal);
    }

    fn read_operator(&mut self, header: Header) -> Packet {
        match self.read_u64(1) {
            0 => self.read_operator_bits(header),
            1 => self.read_operator_count(header),
            _ => unreachable!(),
        }
    }

    fn read_operator_bits(&mut self, header: Header) -> Packet {
        let mut packets = vec![];
        let num_bits: u64 = self.read_u64(15);
        let end = self.bit_pos + (num_bits as usize);
        while self.bit_pos < end {
            packets.push(self.parse_packet());
        }
        let operator = Operator { header, packets };
        return Packet::Operator(operator);
    }

    fn read_operator_count(&mut self, header: Header) -> Packet {
        let mut packets = vec![];
        let n_packets = self.read_u64(11);
        for _ in 0..n_packets {
            packets.push(self.parse_packet());
        }
        let operator = Operator { header, packets };
        return Packet::Operator(operator);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_more_stuff1() {
        let mut pp = PacketParser::new( Vec::from_hex("8A004A801A8002F478").expect("Invalid hex string") );
        let p = pp.parse_packet();
        assert_eq!(p.add_version_numbers(), 16);
    }

    #[test]
    fn test_more_stuff2() {
        let mut pp = PacketParser::new( Vec::from_hex("620080001611562C8802118E34").expect("Invalid hex string") );
        let p = pp.parse_packet();
        assert_eq!(p.add_version_numbers(), 12);
    }

    #[test]
    fn test_more_stuff3() {
        let mut pp = PacketParser::new( Vec::from_hex("C0015000016115A2E0802F182340").expect("Invalid hex string") );
        let p = pp.parse_packet();
        assert_eq!(p.add_version_numbers(), 23);
    }

    #[test]
    fn test_more_stuff4() {
        let mut pp = PacketParser::new( Vec::from_hex("A0016C880162017C3686B18A3D4780").expect("Invalid hex string") );
        let p = pp.parse_packet();
        assert_eq!(p.add_version_numbers(), 31);
    }

    #[test]
    fn test_parse_op_packet_length_type_1() {
        let mut pp = PacketParser::new( Vec::from_hex("EE00D40C823060").expect("Invalid hex string") );
        let p = pp.parse_packet();

        match p {
            Packet::Literal(_) => unreachable!(),
            Packet::Operator(op) => {
                assert_eq!(op.header.version,7);
                assert_eq!(op.header.packet_type,3);
                assert_eq!(op.packets.len(),3);
                for (i,p) in op.packets.iter().enumerate() {
                    match p {
                        Packet::Literal(literal) => assert_eq!(literal.value, (i+1) as u64),
                        Packet::Operator(_) => assert!(false),
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_op_packet_length_type_0() {
        let mut pp = PacketParser::new( Vec::from_hex("38006F45291200").expect("Invalid hex string") );
        let p = pp.parse_packet();

        match p {
            Packet::Literal(_) => unreachable!(),
            Packet::Operator(op) => {
                assert_eq!(op.header.version,1);
                assert_eq!(op.header.packet_type,6);
                assert_eq!(op.packets.len(),2);
                for (i,p) in op.packets.iter().enumerate() {
                    match p {
                        Packet::Literal(literal) => assert_eq!(literal.value, ((i+1)*10) as u64),
                        Packet::Operator(_) => assert!(false),
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_literal_packet() {
        let mut pp = PacketParser::new( Vec::from_hex("D2FE28").expect("Invalid hex string") );
        let p = pp.parse_packet();
        match p {
            Packet::Literal(literal) => assert_eq!(literal.value,2021),
            Packet::Operator(_) => unreachable!(),
        }
    }

    #[test]
    fn test_read_u64() {
        let mut p = PacketParser::new( Vec::from_hex("D2FE28").expect("Invalid hex string") );
        assert_eq!(p.read_u64(3), 0b110); // 3
        assert_eq!(p.read_u64(3), 0b100); // 6
        assert_eq!(p.read_u64(1), 0b1); // 7
        assert_eq!(p.read_u64(4), 0b111); 
        assert_eq!(p.read_u64(1), 0b1);
        assert_eq!(p.read_u64(4), 0b1110);
        assert_eq!(p.read_u64(1), 0b0);
        assert_eq!(p.read_u64(4), 0b0101);
    }
}

fn main() {
    let mut pp = PacketParser::new( Vec::from_hex( include_str!("../inputs/day16.txt").trim() ).expect("Invalid hex string") );
    let p = pp.parse_packet();
    println!("{}",p.add_version_numbers());
    println!("{}",p.do_math());
}

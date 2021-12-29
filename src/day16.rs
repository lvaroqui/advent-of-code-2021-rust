use std::panic;

use crate::Lines;

use crate::bits::Bits;

pub struct Solver {}

enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

pub trait GreaterThan {
    fn greater_than(self) -> bool;
}

impl<T> GreaterThan for T
where
    T: Iterator<Item = i64>,
{
    fn greater_than(mut self) -> bool {
        self.next().unwrap() > self.next().unwrap()
    }
}

pub trait LessThan {
    fn less_than(self) -> bool;
}

impl<T> LessThan for T
where
    T: Iterator<Item = i64>,
{
    fn less_than(mut self) -> bool {
        self.next().unwrap() < self.next().unwrap()
    }
}

pub trait Equal {
    fn equal(self) -> bool;
}

impl<T> Equal for T
where
    T: Iterator<Item = i64>,
{
    fn equal(mut self) -> bool {
        self.next().unwrap() < self.next().unwrap()
    }
}

impl From<u64> for OperatorType {
    fn from(v: u64) -> Self {
        match v {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Minimum,
            3 => OperatorType::Maximum,
            5 => OperatorType::GreaterThan,
            6 => OperatorType::LessThan,
            7 => OperatorType::EqualTo,
            _ => panic!("Unknown operator type"),
        }
    }
}

enum Payload {
    Literal(i64),
    Operator(OperatorType, Vec<Packet>),
}

struct Packet {
    size: usize,
    version: u64,
    payload: Payload,
}

impl Packet {
    fn into_literal(bits: &mut Bits) -> Payload {
        let mut value = 0u64;
        loop {
            let is_last = bits.get(1) == 0;

            let v = bits.get(4);

            value <<= 4;
            value |= v;

            if is_last {
                break;
            }
        }
        Payload::Literal(value as i64)
    }

    fn into_operator(bits: &mut Bits, id: u64) -> Payload {
        let length_type = bits.get(1);
        let mut packets = Vec::new();
        match length_type {
            0 => {
                let mut read = 0;
                let to_read = bits.get(15) as usize;

                while read != to_read {
                    packets.push(Packet::new(bits));
                    read += packets.last().unwrap().size;
                }
            }
            1 => {
                let to_read = bits.get(11) as usize;

                for _ in 0..to_read {
                    packets.push(Packet::new(bits));
                }
            }
            _ => (),
        }
        Payload::Operator(OperatorType::from(id), packets)
    }

    fn new(bits: &mut Bits) -> Packet {
        let start_pos = bits.pos();

        let version = bits.get(3);
        let id = bits.get(3);

        Packet {
            version,
            payload: match id {
                4 => Packet::into_literal(bits),
                _ => Packet::into_operator(bits, id),
            },
            size: bits.pos() - start_pos,
        }
    }

    fn walk<F>(&self, func: &mut F)
    where
        F: FnMut(&Packet),
    {
        func(&self);
        if let Payload::Operator(_, packets) = &self.payload {
            for p in packets.iter() {
                p.walk(func);
            }
        }
    }

    fn value(&self) -> i64 {
        match &self.payload {
            Payload::Literal(v) => *v,
            Payload::Operator(op, packets) => {
                let values = packets.iter().map(|p| p.value());
                match op {
                    OperatorType::Sum => values.sum(),
                    OperatorType::Product => values.product(),
                    OperatorType::Minimum => values.min().unwrap(),
                    OperatorType::Maximum => values.max().unwrap(),
                    OperatorType::GreaterThan => values.greater_than() as i64,
                    OperatorType::LessThan => values.less_than() as i64,
                    OperatorType::EqualTo => values.equal() as i64,
                }
            }
        }
    }
}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, mut lines: Lines) -> String {
        let line = lines.next().unwrap();

        let bytes = (0..line.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&line[i..=i + 1], 16).unwrap())
            .collect::<Vec<_>>();

        let mut bits = Bits::new(&bytes);

        let root = Packet::new(&mut bits);

        let mut version_sum = 0;
        root.walk(&mut |p| version_sum += p.version);

        version_sum.to_string()
    }

    fn solve_part2(self: &mut Self, mut lines: Lines) -> String {
        let line = lines.next().unwrap();

        let bytes = (0..line.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&line[i..=i + 1], 16).unwrap())
            .collect::<Vec<_>>();

        let mut bits = Bits::new(&bytes);

        let root = Packet::new(&mut bits);

        root.value().to_string()
    }
}

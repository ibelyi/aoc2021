use super::common::Step;
use std::fmt;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("31"),
        Step::Second => String::from("54"),
    }
}

const MAX: usize = 200;
struct Stream {
    data: Vec<u8>,
    index: usize,
}

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|v| (0..4).rev().map(move |s| ((v >> s) & 1).to_string()))
                .flatten()
                .skip(self.index)
                .enumerate()
                .map(|(i, v)| if i == MAX {
                    "...".to_string()
                } else if i > MAX {
                    "".to_string()
                } else if i % 80 == 0 {
                    format!("\r\n{}", v)
                } else {
                    v
                })
                .collect::<String>()
        )
    }
}

impl Stream {
    fn new(input: &str) -> Stream {
        let data = input
            .chars()
            .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
            .collect();
        Stream { data, index: 0 }
    }

    fn next(&mut self, bits: usize) -> usize {
        self.index += bits;
        if self.index >= self.data.len() * 4 {
            return 0;
        }
        (((self.index - bits) / 4..=(self.index - 1) / 4)
            .fold(0u64, |s, v| (s << 4) + self.data[v] as u64)
            >> (3 - (self.index - 1) % 4)) as usize
            & ((1 << bits) - 1)
    }

    fn index(&self) -> usize {
        self.index
    }

    fn has(&self, bits: usize) -> bool {
        self.index + bits < self.data.len() * 4
    }
}

enum Content {
    Literal(u64),
    SubPackets(usize, Vec<Packet>),
}

struct Packet {
    version: usize,
    content: Content,
}

impl Packet {
    fn versions(&self) -> usize {
        self.version
            + match &self.content {
                Content::Literal(_) => 0,
                Content::SubPackets(_, subs) => subs.iter().map(|p| p.versions()).sum(),
            }
    }

    fn result(&self) -> u64 {
        match &self.content {
            Content::Literal(value) => *value,
            Content::SubPackets(id, subs) => match id {
                0 => subs.iter().map(|p| p.result()).sum(),
                1 => subs.iter().map(|p| p.result()).product(),
                2 => subs.iter().map(|p| p.result()).min().unwrap(),
                3 => subs.iter().map(|p| p.result()).max().unwrap(),
                5 => {
                    if subs[0].result() > subs[1].result() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if subs[0].result() < subs[1].result() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if subs[0].result() == subs[1].result() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("Unknown packet id!"),
            },
        }
    }
}

fn read_packet(stream: &mut Stream) -> Option<Packet> {
    if !stream.has(11) {
        return None;
    }
    let version = stream.next(3);
    let id = stream.next(3);
    if id == 4 {
        let mut literal = 0;
        loop {
            let more = stream.next(1);
            literal = (literal << 4) + stream.next(4) as u64;
            if more == 0 {
                break;
            }
        }
        return Some(Packet {
            version,
            content: Content::Literal(literal),
        });
    }
    let mut sub_packets = vec![];
    if stream.next(1) == 0 {
        let total = stream.next(15) + stream.index();
        while stream.index() < total {
            if let Some(packet) = read_packet(stream) {
                sub_packets.push(packet);
            }
        }
    } else {
        for _ in 0..stream.next(11) {
            if let Some(packet) = read_packet(stream) {
                sub_packets.push(packet);
            }
        }
    }
    Some(Packet {
        version,
        content: Content::SubPackets(id, sub_packets),
    })
}

fn count_versions(data: &str) -> usize {
    let mut stream = Stream::new(data);
    let mut count = 0;
    while let Some(packet) = read_packet(&mut stream) {
        count += packet.versions();
    }
    count
}

fn count_result(data: &str) -> u64 {
    let mut stream = Stream::new(data);
    if let Some(packet) = read_packet(&mut stream) {
        packet.result()
    } else {
        0
    }
}

pub fn solution(step: &Step, input: &Vec<String>) -> String {
    let data = &input[0];
    match step {
        Step::First => count_versions(&data).to_string(),
        Step::Second => count_result(&data).to_string(),
    }
}

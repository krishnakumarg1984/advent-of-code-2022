use bytes::BytesMut;

pub fn main() {
    let mut bits = include_str!("../input.txt")
        .trim()
        .chars()
        .flat_map(|c| {
            let n = c.to_digit(16).unwrap();
            (0..4).map(move |i| (1 & n >> (3 - i)) as u8)
        })
        .collect();
    println!("{}", packet(&mut bits)[0]);
}

fn packet(bits: &mut BytesMut) -> Vec<usize> {
    let typ = num(&bits.split_to(6)[3..]);
    if typ == 4 {
        let mut num_bits = vec![];
        (0..)
            .take_while(|_| {
                let bits = bits.split_to(5);
                num_bits.extend(&bits[1..]);
                bits[0] == 1
            })
            .count();
        return vec![num(&num_bits)];
    }

    let mut nums = vec![];
    if bits.split_to(1)[0] == 0 {
        let len = num(&bits.split_to(15)) as usize;
        let mut bits = bits.split_to(len);
        while !bits.is_empty() {
            nums.extend_from_slice(&packet(&mut bits));
        }
    } else {
        (0..num(&bits.split_to(11))).for_each(|_| nums.extend_from_slice(&packet(bits)));
    }

    match typ {
        0 => vec![nums.iter().sum()],
        1 => vec![nums.iter().product()],
        2 => vec![*nums.iter().min().unwrap()],
        3 => vec![*nums.iter().max().unwrap()],
        5 => vec![(nums[0] > nums[1]) as usize],
        6 => vec![(nums[0] < nums[1]) as usize],
        7 => vec![(nums[0] == nums[1]) as usize],
        _ => unreachable!(),
    }
}

fn num(b: &[u8]) -> usize {
    b.iter()
        .enumerate()
        .map(|(i, &x)| (x as usize) << (b.len() - i - 1))
        .sum()
}

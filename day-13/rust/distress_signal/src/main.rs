use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum PacketData {
    List(Vec<PacketData>),
    Integer(u32),
}

impl Display for PacketData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_string(self).unwrap();
        write!(f, "{}", s)
    }
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        match get_data_order(self, other) {
            DataOrder::Equal => true,
            _ => false,
        }
    }
}

impl Eq for PacketData {}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match get_data_order(self, other) {
            DataOrder::Equal => Ordering::Equal,
            DataOrder::RightOrder => Ordering::Less,
            DataOrder::OutOfOrder => Ordering::Greater,
        }
    }
}

#[derive(Debug)]
enum DataOrder {
    RightOrder,
    OutOfOrder,
    Equal,
}

fn match_two_numbers(n1: &u32, n2: &u32) -> DataOrder {
    if n1 == n2 {
        DataOrder::Equal
    } else if n1 < n2 {
        DataOrder::RightOrder
    } else {
        DataOrder::OutOfOrder
    }
}

fn match_number_with_list(n1: &u32, v2: &Vec<PacketData>) -> DataOrder {
    if v2.is_empty() {
        return DataOrder::OutOfOrder;
    }
    let e2 = v2.first().unwrap();
    let first = PacketData::Integer(*n1);
    match get_data_order(&first, e2) {
        DataOrder::Equal => {
            if v2.get(1).is_some() {
                DataOrder::RightOrder
            } else {
                DataOrder::Equal
            }
        }
        DataOrder::RightOrder => DataOrder::RightOrder,
        DataOrder::OutOfOrder => DataOrder::OutOfOrder,
    }
}

fn match_list_with_number(v1: &Vec<PacketData>, n2: &u32) -> DataOrder {
    if v1.is_empty() {
        return DataOrder::RightOrder;
    }
    let e1 = v1.first().unwrap();
    let second = PacketData::Integer(*n2);
    match get_data_order(e1, &second) {
        DataOrder::OutOfOrder => DataOrder::OutOfOrder,
        DataOrder::RightOrder => DataOrder::RightOrder,
        DataOrder::Equal => {
            if v1.get(1).is_some() {
                DataOrder::OutOfOrder
            } else {
                DataOrder::Equal
            }
        }
    }
}

fn match_list_with_list(v1: &Vec<PacketData>, v2: &Vec<PacketData>) -> DataOrder {
    let mut i1 = v1.iter();
    let mut i2 = v2.iter();
    loop {
        let e1 = i1.next();
        let e2 = i2.next();
        if e1.is_none() {
            return DataOrder::RightOrder;
        }
        if e2.is_none() {
            return DataOrder::OutOfOrder;
        }
        let e1 = e1.unwrap();
        let e2 = e2.unwrap();
        match get_data_order(e1, e2) {
            DataOrder::Equal => continue,
            DataOrder::OutOfOrder => return DataOrder::OutOfOrder,
            DataOrder::RightOrder => return DataOrder::RightOrder,
        }
    }
}

fn get_data_order(first: &PacketData, second: &PacketData) -> DataOrder {
    match (first, second) {
        (PacketData::Integer(n1), PacketData::Integer(n2)) => match_two_numbers(n1, n2),
        (PacketData::Integer(n1), PacketData::List(v2)) => match_number_with_list(n1, v2),
        (PacketData::List(v1), PacketData::Integer(n2)) => match_list_with_number(v1, n2),
        (PacketData::List(v1), PacketData::List(v2)) => match_list_with_list(v1, v2),
    }
}

fn main() {
    let file_name = "input.txt";
    let content = std::fs::read_to_string(file_name).unwrap();
    let mut data = content
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_str::<PacketData>(s).unwrap())
        .collect::<Vec<_>>();

    let sum = data
        .clone()
        .chunks(2)
        .enumerate()
        .map(|(i, chunk)| if chunk[0] < chunk[1] { i + 1 } else { 0 })
        .sum::<usize>();
    println!("{sum}");
    let dp1 = "[[2]]";
    let dp2 = "[[6]]";
    let divider_packet_1 = serde_json::from_str(dp1).unwrap();
    let divider_packet_2 = serde_json::from_str(dp2).unwrap();
    data.push(divider_packet_1);
    data.push(divider_packet_2);
    data.sort();
    let dp1_idx = data.iter().position(|d| d.to_string().eq(dp1)).unwrap();
    let dp2_idx = data.iter().position(|d| d.to_string().eq(dp2)).unwrap();
    println!("{}", (dp1_idx + 1) * (dp2_idx + 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_data() {
        let s = "[[1],[2,3,4]]";
        let data1 = serde_json::from_str::<PacketData>(s).unwrap();
        let s = "[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let data2 = serde_json::from_str::<PacketData>(s).unwrap();
        println!("{:?}", get_data_order(&data1, &data2));
        assert_eq!(1, 0);
    }
}

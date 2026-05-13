use std::{
    fmt::write,
    io::{Write, stdin, stdout},
};

fn main() {
    println!("Задание 1");
    let ips_first = obtain_ips("IP-адреса: ");
    for ip in &ips_first {
        let ip_bytes = ip.to_be_bytes();
        println!(
            "{} - класс {}",
            ip_dec_presentation(ip_bytes),
            classify_ip(ip_bytes[0])
        )
    }

    println!("Задание 2");
    let masks = obtain_masks("Маски сетей: ");
    let mut networks = Vec::new();

    for (i, mask) in masks.iter().enumerate() {
        let ip = ips_first[i];
        let network = ip & mask;
        networks.push(network);
        let nw_bytes = network.to_be_bytes();
        println!(
            "Адрес сети в двоичном виде: \"{}\", в десятичном виде: \"{}\"",
            ip_bin_presentation(nw_bytes),
            ip_dec_presentation(nw_bytes)
        )
    }

    println!("Задание 3");
    let mut numbers = Vec::new();
    for (i, mask) in masks.iter().enumerate() {
        let ip = ips_first[i];
        let num = ip & (!mask);
        numbers.push(num);
        let num_bytes = num.to_be_bytes();
        println!(
            "Адрес узла в двоичном виде: \"{}\", в десятичном виде: \"{}\"",
            ip_bin_presentation(num_bytes),
            ip_dec_presentation(num_bytes)
        )
    }

    println!("Задание 4");
    numbers.iter().for_each(|n| {
        let num_bytes = n.to_be_bytes();
        println!(
            "Номер узла в двоичном виде: \"{}\", в десятичном виде: {}",
            ip_bin_presentation(num_bytes),
            n
        )
    });

    println!("Задание 5");
    for (i, nw) in networks.iter().enumerate() {
        let broadcast_ip = (!masks[i] | nw).to_be_bytes();
        println!(
            "Широковещательный IP-адрес в двоичном виде: \"{}\", в десятичном виде: \"{}\"",
            ip_bin_presentation(broadcast_ip),
            ip_dec_presentation(broadcast_ip)
        )
    }

    println!("Дополнительное задание");
    let numbers = obtain_numbers("Количества узлов в каждой подсети: ");
    for (i, nw) in networks.iter().enumerate() {
        println!("Задача {}", i + 1);
        let mask = masks[i];

        let ips_in_nw = numbers[i] + 2;
        let matter_bytes = (ips_in_nw as f64).log2().ceil() as u8;
        let new_prefix = 32 - matter_bytes;
        let newmask = prefix_to_mask(new_prefix);

        let mut new_network = nw & newmask;

        let ips_in_nw = 2u32.pow(matter_bytes as u32);
        for j in 1..=2u8.pow(new_prefix as u32 - mask.leading_ones()) {
            present_subnet(j, new_network, new_prefix, ips_in_nw);
            new_network += ips_in_nw;
        }
    }
}

fn present_subnet(ctr: u8, new_network: u32, newmask: u8, ips_in_nw: u32) {
    println!(
        "Подсеть {}:\n\t— Адрес подсети: {}/{};\n\t— Блок адресов: {}—{}",
        ctr,
        ip_dec_presentation(new_network.to_be_bytes()),
        newmask,
        ip_dec_presentation(new_network.to_be_bytes()),
        ip_dec_presentation((new_network + ips_in_nw - 1).to_be_bytes()),
    );
}

fn classify_ip(first_byte: u8) -> &'static str {
    match first_byte {
        0..=127 => "A",
        128..=191 => "B",
        192..=223 => "C",
        224..=239 => "D",
        _ => "E",
    }
}

fn obtain_ips(prompt: &str) -> Vec<u32> {
    let inp = my_input(prompt);
    let mut res = Vec::new();
    for ip in inp.trim().split_whitespace() {
        let mut ip_octets = ip.split(".");
        let ip_bytes: [u8; 4] = std::array::from_fn(|_| ip_octets.next().unwrap().parse().unwrap());
        res.push(u32::from_be_bytes(ip_bytes));
    }
    return res;
}

fn obtain_masks(prompt: &str) -> Vec<u32> {
    let inp = my_input(prompt);
    inp.trim()
        .split_whitespace()
        .map(|m| prefix_to_mask(m.parse::<u8>().unwrap()))
        .collect()
}

fn my_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut inp = String::new();
    stdin().read_line(&mut inp).unwrap();
    return inp;
}

fn prefix_to_mask(m: u8) -> u32 {
    assert!(m <= 32);

    if m == 0 { 0 } else { u32::MAX << (32 - m) }
}

fn ip_dec_presentation(ip: [u8; 4]) -> String {
    ip.map(|o| o.to_string()).join(".")
}

fn ip_bin_presentation(ip: [u8; 4]) -> String {
    ip.map(|o| format!("{:08b}", o)).join(".")
}

fn obtain_numbers(prompt: &str) -> Vec<u32> {
    let inp = my_input(prompt);
    inp.trim()
        .split_whitespace()
        .map(|m| m.parse().unwrap())
        .collect()
}

// enum ArrayFromIterError {
//     TooManyItems,
//     TooFewItems,
// }

// fn array_from_iter<I, T, const N: usize>(mut it: I) -> Result<[T; N], ArrayFromIterError>
// where
//     I: Iterator<Item = T>
// {
//     let mut res: [T; N];
//     let mut ctr = 0;
//     for (i, el) in it.enumerate() {
//         ctr += 1;
//         if ctr > N {
//             return Err(ArrayFromIterError::TooManyItems);
//         } else {
//             res[i] = match it.next() {
//                 Some(v) => v,
//                 None => return Err(ArrayFromIterError::TooFewItems)
//             };
//         }
//     }
//     return todo!();
// }

// fn iter_to_array<T, I, const N: usize>(mut it: I) -> [T; N]
// where
//     I: ExactSizeIterator<Item = T>,
// {
//     assert_eq!(it.len(), N);
//     std::array::from_fn(|_| it.next().unwrap())
// }

// pub fn classify_ip(first_byte: u8) -> &'static str {
//     if first_byte < 0b10000000 {
//         "A"
//     } else if first_byte < 0b11000000 {
//         "B"
//     } else if first_byte < 0b11100000 {
//         "C"
//     } else if first_byte < 0b11110000 {
//         "D"
//     } else {
//         "E"
//     }
// }

// struct IpAddress {
//     address: u32
// }

// enum IpParseError {
//     TooMuchBytes,
//     TooFewBytes,
//     TooBigByte,
//     InvalidByte,
// }

// impl IpAddress {
//     fn from_str(s: &str) -> Result<Self, IpParseError> {
//         let mut ip: Vec<&str> = s.split(".").collect();
//         if
//         todo!()
//     }
// }

// struct IpNetwork {
//     address: IpAddress,
//     mask_prefix: u8
// }
// impl IpNetwork {
//     fn from_ip_and_pask_prefix(ip: IpAddress, mp: u8) -> Self {
//         todo!()
//     }

// }

#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn ip_test_2() {
    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));
}

#[derive(Debug)]
enum IpAddr1 {
    V4(String),
    V6(String),
}

fn ip_test_1() {
    let home = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback = IpAddr1::V6(String::from("::1"));
}

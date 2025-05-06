enum Coin {
    Paise(i32),
    Rupee(i32),
    Nothing,
}

fn value_in_rupees(coin: Coin) -> f32 {
    match coin {
        Coin::Paise(x) => (x as f32) / 100.0,
        Coin::Rupee(x) => x as f32,
        Coin::Nothing => 0 as f32,
    }
}

fn main() {
    let x = value_in_rupees(Coin::Paise(100));

    println!("Value of x is {}", x);

    let y = Coin::Rupee(100);
    let _z = Coin::Nothing;

    if let Coin::Nothing = y {
        println!("{}", 0);
    } else if let Coin::Rupee(x) = y {
        println!("{}", x);
    } else {
        println!("{}", value_in_rupees(y));
    }
}

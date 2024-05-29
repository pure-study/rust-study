#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    test_match_clause(Coin::Dime);
    test_match_clause(Coin::Quarter(UsState::Alaska));
    println!("=============");
    test_if_let(Coin::Nickel);
    test_if_let(Coin::Penny);
    test_if_let(Coin::Quarter(UsState::Alabama));
}

// Option 1:
fn test_match_clause(coin: Coin) {
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("(match_clause) State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("(match_clause) {}", count);
}

// Option 2:
fn test_if_let(coin: Coin) {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("(if_let) State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("(if_let) {}", count);
}
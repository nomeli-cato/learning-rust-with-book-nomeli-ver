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
    println!("if let");

    {
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum is configured to be {}", max),
            _ => (),
        }
    }
    {
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        }
    }
    {
        let mut count = 0;
        let coin = Coin::Quarter(UsState::Alabama);
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,
        }
        
        println!("{}", count);
        
    }
    
    {        
        let coin = Coin::Quarter(UsState::Alabama);
        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }
        
        println!("{}", count);
    }
}

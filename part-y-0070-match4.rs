fn main() {

    let our_num: i32 = -4000;

    match our_num {

        on if on % 2 == 0 && on > 0 => println!("{on} is even and positive"),

        on if on % 2 != 0 &&  on > 0 => println!("{on} is odd and positive"),

        _ => println!("We need positive values! Don't tell us a negative value!")
        
    }

}

// We need positive values! Don't tell us a negative value!

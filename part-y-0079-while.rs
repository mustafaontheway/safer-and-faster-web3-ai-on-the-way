fn main() {

    let mut seconds: u8 = 0;

    while seconds <= 60 {

        seconds += 5;

        println!("Seconds: {seconds}");

        if seconds == 20 {

            println!("We don't like number '20'. Pass it!");

            continue;
        } 

        if seconds == 60 {

            break;
        }
    }

}

// Seconds: 5
// Seconds: 10
// Seconds: 15
// Seconds: 20
// We don't like number '20'. Pass it!
// Seconds: 25
// Seconds: 30
// Seconds: 35
// Seconds: 40
// Seconds: 45
// Seconds: 50
// Seconds: 55
// Seconds: 60

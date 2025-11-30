use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use rand::{SeedableRng, rngs::StdRng};

use crate::mimetize::{HashMimetize, Mimetize, RngState};

mod mimetize;

fn main() {
    let f = |a: String| {
        sleep(Duration::from_secs(2));
        a.len()
    };

    let mut m = HashMimetize::new(f);

    let seed = [42; 32];

    let mut rng1: RngState<i32> = RngState::new(StdRng::from_seed(seed));
    let mut rng2 = RngState::new(StdRng::from_seed(seed));

    assert_eq!(rng1.call(0), rng2.call(0), "Index 0 tem que dar match");
    assert_eq!(rng1.call(5), rng2.call(5), "Index 5 tem que dar match");
    assert_eq!(
        rng1.call(100),
        rng2.call(100),
        "Index 100 tem que dar match"
    );

    let instant = Instant::now();
    let x = rng1.call(10000000);
    println!("{x}");
    println!(
        "Tempo chamado para a call 10000000 de rng: {:?}",
        instant.elapsed()
    );

    println!("aaa");

    let instant = Instant::now();
    let x = m.call(String::from("FIJQIFOQWJFOQIWFJWQIFWJQ"));
    println!("{x}");
    println!("Tempo chamado primeira call: {:?}", instant.elapsed());

    let instant = Instant::now();
    let x = m.call(String::from("FIJQIFOQWJFOQIWFJWQIFWJQ"));
    println!("{x}");
    println!("Tempo chamado segunda call: {:?}", instant.elapsed());

    let instant = Instant::now();
    let x = m.call(String::from("aaaaaa"));
    println!("{x}");
    println!(
        "Tempo chamado terceira call com outro argumento: {:?}",
        instant.elapsed()
    );
}

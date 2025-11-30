use crate::mimetize::Mimetize;

mod mimetize;

fn main() {
    let f1 = |()| {
        println!("Hello!");

        true
    };

    let mut m1 = Mimetize::new(f1);

    for _ in 0..5 {
        m1.call(());
    }

    let f2 = |x: i32| {
        static mut Y: i32 = 0;

        unsafe {
            Y += x;
            return Y;
        }
    };

    let mut m2 = Mimetize::new(f2);

    for i in 0..5 {
        let y = m2.call(2);
        println!("i: {i}, Y: {y}")
    }
}

#[cfg(test)]
mod test {

    use std::thread::sleep;
    use std::time::Duration;

    use rand::Rng;
    use rand::SeedableRng;
    use rand::rng;
    use rand::rngs::StdRng;

    use crate::mimetize::Mimetize;

    #[test]
    fn mimetizar_random_sem_seed() {
        let random = |()| {
            let mut r = rng();
            r.random::<i32>()
        };

        let mut m2 = Mimetize::new(random);

        assert!(m2.call(()) != random(()));
    }

    #[test]
    fn mimetizar_random_com_seed() {
        let seed = [42; 32];

        let random = |seed: [u8; 32]| {
            let mut r = StdRng::from_seed(seed);
            r.random::<i32>()
        };

        let mut m2 = Mimetize::new(random);

        assert_eq!(m2.call(seed), random(seed));
    }

    #[test]
    fn mimetizar_funcao_com_sleep() {
        // Função mimetizada
        let f = |a: String| {
            sleep(Duration::from_secs(2));
            a.len()
        };
        // Mimetize
        let mut m = Mimetize::new(f);

        // Demora 2s
        let x = m.call(String::from("Argumento"));
        // Instantâneo
        let y = m.call(String::from("Argumento"));

        assert_eq!(x, y);
    }
}

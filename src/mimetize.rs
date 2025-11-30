use std::{collections::HashMap, hash::Hash};

use rand::{Rng, distr::StandardUniform, rngs::StdRng};

/// Mimetize é uma trait que quem for mimetizar alguma coisa deve implementar.
/// Na prática, não faz muita coisa, mas é mais pra demarcar quem tá chamando a função X
/// mimetizada.
///
/// I, O são tipos genéricos, pra quem for implementar definir o que eles significam.
pub trait Mimetize<I, O: Clone> {
    fn call(&mut self, i: I) -> O;
}

/// Esse Hashmimetize é a implementação pra qualquer função "normal", que apenas recebe uma entrada
/// I, modifica ela e transforma em uma saída do tipo O. Basicamente todas as funções "puras" com 1
/// argumento são mimetizadas por essa classe/struct.
pub struct HashMimetize<I: Hash + Eq + Clone, O: Clone, F: Fn(I) -> O> {
    args: HashMap<I, O>,
    f: Box<F>,
}

impl<I: Hash + Eq + Clone, O: Clone, F: Fn(I) -> O> HashMimetize<I, O, F> {
    /// Implementação do método de inicialização da mimetize, ela guarda uma função e um hashmap
    /// pra todos os argumentos que já foram salvos.
    pub fn new(f: F) -> Self {
        Self {
            args: HashMap::new(),
            f: Box::new(f),
        }
    }
}

/// Aqui é a implementação da interface Mimetize para o HashMimetize
impl<I: Hash + Eq + Clone, O: Clone, F: Fn(I) -> O> Mimetize<I, O> for HashMimetize<I, O, F> {
    fn call(&mut self, i: I) -> O {
        self.args
            .entry(i.clone())
            // Esse (self.f)(i) é chamando a função com o argumento i.
            .or_insert_with(|| (self.f)(i))
            .clone()
    }
}

/// No caso do random generator, não tem como chegar na mesma conclusão, porque a própria função do
/// random depende da seed que lhe foi passada, e não do argumento da função. Desse modo, os atributos mudam.
/// Apenas salvo todas as chamadas de rng em um Vec<O> (O é de output). A idéia aqui é que a
/// mimetize será uma função F: Inteiro -> O, e o inteiro vai definir qual das chamadas da função
/// rng você quer acessar.
///
/// # Exemplo
///
/// Caso você queira acessar a rng 1000, em vez de chamar .call() (da interface) 1000 vezes, você
/// vai chamar .call(1000).
pub struct RngState<O> {
    args: Vec<O>,
    seed: StdRng,
}

impl<O: Clone> RngState<O> {
    /// Para instanciar esse mimetizador, é preciso apenas da seed
    pub fn new(seed: StdRng) -> Self {
        Self {
            args: Vec::new(),
            seed,
        }
    }
}

impl<O: Clone> Mimetize<usize, O> for RngState<O>
where
    StandardUniform: rand::distr::Distribution<O>,
{
    /// A implementação fica bem simples, apenas veja qual a posição foi pedida, caso já tenha no
    /// cache, apenas retorne, caso você tenha pedido uma função muito a frente, ele vai executar
    /// LEN - I vezes a função, para chegar na execução pedida.
    fn call(&mut self, i: usize) -> O {
        while self.args.len() <= i {
            self.args.push(self.seed.random());
        }

        self.args[i].clone()
    }
}

// AI generated tests
#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn test_determinism_same_seed() {
        let seed = [42; 32];

        let mut rng1: RngState<i32> = RngState::new(StdRng::from_seed(seed));
        let mut rng2: RngState<i32> = RngState::new(StdRng::from_seed(seed));

        // They should produce the exact same sequence for the same indices
        assert_eq!(rng1.call(0), rng2.call(0), "Index 0 should match");
        assert_eq!(rng1.call(5), rng2.call(5), "Index 5 should match");
        assert_eq!(rng1.call(100), rng2.call(100), "Index 100 should match");
    }

    #[test]
    fn test_determinism_different_seed() {
        let seed1 = [1; 32];
        let seed2 = [2; 32];

        let mut rng1: RngState<i32> = RngState::new(StdRng::from_seed(seed1));
        let mut rng2: RngState<i32> = RngState::new(StdRng::from_seed(seed2));

        // Extremely unlikely these will be equal
        assert_ne!(rng1.call(0), rng2.call(0));
    }

    #[test]
    fn test_caching_idempotency() {
        let seed = [55; 32];
        let mut rng: RngState<i32> = RngState::new(StdRng::from_seed(seed));

        // Generate a value for index 10
        let first_call = rng.call(10);

        // Call it again. It should return the cached value, not a new random number.
        let second_call = rng.call(10);

        assert_eq!(
            first_call, second_call,
            "Subsequent calls to the same index must return cached value"
        );
    }

    #[test]
    fn test_filling_gaps() {
        let seed = [10; 32];
        let mut rng: RngState<i32> = RngState::new(StdRng::from_seed(seed));

        // Requesting index 4 should generate indices 0, 1, 2, 3, 4.
        // Total length should be 5.
        let _ = rng.call(4);

        assert_eq!(
            rng.args.len(),
            5,
            "Vector should have expanded to size 5 to accommodate index 4"
        );
    }

    #[test]
    fn test_sequential_generation() {
        let seed = [99; 32];
        let mut rng: RngState<i32> = RngState::new(StdRng::from_seed(seed));

        let v0 = rng.call(0);
        let v1 = rng.call(1);
        let v2 = rng.call(2);

        // Just ensuring they don't crash and generate distinct values (mostly)
        assert_ne!(v0, v1);
        assert_ne!(v1, v2);
        assert_eq!(rng.args.len(), 3);
    }

    #[test]
    fn test_different_types() {
        let seed = [7; 32];

        // Test with Boolean
        let mut bool_rng: RngState<bool> = RngState::new(StdRng::from_seed(seed));
        let b = bool_rng.call(0);
        assert!(b == true || b == false);

        // Test with f64
        let mut float_rng: RngState<f64> = RngState::new(StdRng::from_seed(seed));
        let f = float_rng.call(0);
        assert!(f >= 0.0 && f < 1.0);
    }
}

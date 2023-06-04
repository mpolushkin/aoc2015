use std::time::Instant;

use super::Challenge;

pub struct Day20 {
    input: u32,
}

impl Challenge for Day20 {
    const DAY: u8 = 20;

    type Part1Solution = u32;
    type Part2Solution = u32;

    fn new(input: &str) -> Self {
        Self {
            input: input.trim().parse::<u32>().unwrap(),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        let start = Instant::now();
        let mut progress = 1u32;
        for (house, num_presents) in PresentsUsingPrimes::new() {
            if num_presents >= self.input {
                println!("took {:?}", start.elapsed());
                return house;
            }
            if num_presents >= self.input * progress / 100 {
                println!("{:3}: {}", progress, house);
                progress += 1;
            }
        }
        0
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        let stamina = 50;
        let multiplier = 11;
        let min_house = (self.input as f64
            / multiplier as f64
            / (1..=stamina).map(|n| 1. / (n as f64)).sum::<f64>()) as u32;
        for house in min_house.. {
            let num_presents = presents_stamina(house, stamina, 11);
            if num_presents >= self.input {
                return house;
            }
        }
        0
    }
}

fn presents_naive(house: u32) -> u32 {
    (1..=house)
        .map(|i| if house % i == 0 { i * 10 } else { 0 })
        .sum()
}

struct PresentsUsingPrimes {
    house: u32,
    primes: Vec<u32>,
}

impl PresentsUsingPrimes {
    fn new() -> Self {
        Self {
            house: 1,
            primes: Vec::new(),
        }
    }

    fn prime_factors(&mut self) -> Vec<(u32, u32)> {
        let mut prime_factors = Vec::new();
        let mut x = self.house;
        for prime in self.primes.iter().copied() {
            let mut power = 0;
            while x % prime == 0 {
                x /= prime;
                power += 1;
            }
            if power > 0 {
                prime_factors.push((prime, power))
            }
        }
        if prime_factors.len() == 0 {
            if x == 1 {
                // 1 is a special case
                prime_factors.push((2, 0));
            } else {
                // found a new prime
                prime_factors.push((x, 1));
                self.primes.push(x);
            }
        }
        prime_factors
    }

    fn all_divisors_from_prime_factors<'a>(
        &'a self,
        prime_factors: &'a Vec<(u32, u32)>,
    ) -> AllDivisorsFromPrimeFactors {
        AllDivisorsFromPrimeFactors::new(prime_factors)
    }

    fn sum_of_factors(&mut self) -> u32 {
        let prime_factors = self.prime_factors();
        self.all_divisors_from_prime_factors(&prime_factors).sum()
    }
}

impl Iterator for PresentsUsingPrimes {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let house = self.house;
        let presents = 10 * self.sum_of_factors();
        self.house += 1;
        Some((house, presents))
    }
}

struct AllDivisorsFromPrimeFactors<'a> {
    prime_factors: &'a Vec<(u32, u32)>,
    current_powers: Vec<u32>,
}

impl<'a> AllDivisorsFromPrimeFactors<'a> {
    fn new(prime_factors: &'a Vec<(u32, u32)>) -> Self {
        Self {
            prime_factors,
            current_powers: vec![0; prime_factors.len()],
        }
    }

    fn current_factor(&self) -> Option<u32> {
        if self.current_powers.len() == 0 {
            None
        } else {
            Some(
                self.prime_factors
                    .iter()
                    .zip(&self.current_powers)
                    .map(|(&(prime, _), &current_power)| prime.pow(current_power))
                    .product(),
            )
        }
    }

    fn advance(&mut self) {
        for (&(_, max_power), current_power) in
            self.prime_factors.iter().zip(&mut self.current_powers)
        {
            let next_power = *current_power + 1;
            if next_power <= max_power {
                *current_power = next_power;
                return;
            } else {
                *current_power = 0;
            }
        }
        self.current_powers.clear();
    }
}

impl<'a> Iterator for AllDivisorsFromPrimeFactors<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let factor = self.current_factor()?;
        self.advance();
        Some(factor)
    }
}

fn presents_stamina(house: u32, stamina: u32, multiplier: u32) -> u32 {
    (1..=stamina)
        .filter_map(|divisor| {
            if house % divisor == 0 {
                Some(house / divisor)
            } else {
                None
            }
        })
        .sum::<u32>()
        * multiplier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive() {
        assert_eq!(presents_naive(1), 10);
        assert_eq!(presents_naive(2), 30); //    2, 1
        assert_eq!(presents_naive(3), 40); //    3, 1
        assert_eq!(presents_naive(4), 70); //    4, 1, 2
        assert_eq!(presents_naive(5), 60); //    5, 1
        assert_eq!(presents_naive(6), 120); //   6, 1, 2, 3
        assert_eq!(presents_naive(7), 80); //    7, 1
        assert_eq!(presents_naive(8), 150); //   8, 1, 2, 4
        assert_eq!(presents_naive(9), 130); //   9, 1, 3
        assert_eq!(presents_naive(10), 180); // 10, 1, 2, 5
        assert_eq!(presents_naive(11), 120); // 11, 1
        assert_eq!(presents_naive(12), 280); // 12, 1, 2, 3, 4, 6

        assert_eq!(presents_naive(36), 910); // 36, 1, 2, 3, 4, 6, 9, 12, 18
    }

    #[test]
    fn test_presents_using_primes() {
        let mut presents_using_primes = PresentsUsingPrimes::new();
        assert_eq!(presents_naive(1), presents_using_primes.next().unwrap().1);
        assert_eq!(presents_naive(2), presents_using_primes.next().unwrap().1);
        assert_eq!(presents_naive(3), presents_using_primes.next().unwrap().1);
        assert_eq!(presents_naive(4), presents_using_primes.next().unwrap().1);
        assert_eq!(presents_naive(5), presents_using_primes.next().unwrap().1);
        assert_eq!(presents_naive(6), presents_using_primes.next().unwrap().1);
        assert_eq!(presents_naive(7), presents_using_primes.next().unwrap().1);
        assert_eq!(presents_naive(8), presents_using_primes.next().unwrap().1);
        assert_eq!(presents_naive(9), presents_using_primes.next().unwrap().1);
        assert_eq!(presents_naive(10), presents_using_primes.next().unwrap().1);
        assert_eq!(presents_naive(11), presents_using_primes.next().unwrap().1);
    }

    #[test]
    fn test_limited_stamina() {
        assert_eq!(presents_stamina(1, 5, 10), 10);
        assert_eq!(presents_stamina(2, 5, 10), 30); //    2, 1
        assert_eq!(presents_stamina(3, 5, 10), 40); //    3, 1
        assert_eq!(presents_stamina(4, 5, 10), 70); //    4, 1, 2
        assert_eq!(presents_stamina(5, 5, 10), 60); //    5, 1
        assert_eq!(presents_stamina(6, 5, 10), 110); //   6, 2, 3       not: 1
        assert_eq!(presents_stamina(7, 5, 10), 70); //    7             not: 1
        assert_eq!(presents_stamina(8, 5, 10), 140); //   8, 2, 4       not: 1
        assert_eq!(presents_stamina(9, 5, 10), 120); //   9, 3          not: 1
        assert_eq!(presents_stamina(10, 5, 10), 170); // 10, 2, 5       not: 1
        assert_eq!(presents_stamina(11, 5, 10), 110); // 11             not: 1
        assert_eq!(presents_stamina(12, 5, 10), 250); // 12, 3, 4, 6    not: 1, 2

        assert_eq!(presents_stamina(36, 5, 10), 750); // 36, 9, 12, 18   not:  1, 2, 3, 4, 6
    }
}

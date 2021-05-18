use std::{
    convert::TryInto,
    time::{Duration, Instant},
};
use bit_vec::BitVec;
use phf::phf_map;

const RESULTS_DICTIONARY: phf::Map<u64, u32> = phf_map! {
                10_u64 => 4,              // Historical data for validating our results - the number of primes
               100_u64 => 25,             // to be found under some limit, such as 168 primes under 1000
             1_000_u64 => 168,
            10_000_u64 => 1229,
           100_000_u64 => 9592,
         1_000_000_u64 => 78498,
        10_000_000_u64 => 664579,
       100_000_000_u64 => 5761455,
     1_000_000_000_u64 => 50847534,
    10_000_000_000_u64 => 455052511,
};

struct PrimeSieve {
    sieve_size: usize,
    bits: BitVec,
}

impl PrimeSieve {
    fn validate_results(&self) -> bool {
        match RESULTS_DICTIONARY.get(&(self.sieve_size as u64)) {
            Some(result) => (*result as usize) == self.count_primes(),
            None => false,
        }
    }

    fn new(sieve_size: usize) -> Self {
        Self {
            sieve_size,
            bits: BitVec::from_elem(sieve_size, true),
        }
    }

    fn run_sieve(&mut self) {
        let mut factor = 3;
        let limit = 1 + (self.sieve_size as f64).sqrt() as usize;

        while factor <= limit {
            // Find the next prime value
            for num in (factor..self.sieve_size).step_by(2) {
                if self.bits[num] {
                    factor = num;
                    break;
                }
            }
            for num in (factor*factor..self.sieve_size).step_by(factor*2) {
                self.bits.set(num, false);
            }
            factor += 2;
        }
    }

    fn print_results(&self, show_results: bool, duration: Duration, passes: u32) {
        if show_results {
            print!("2, ");
        }

        let mut count = match self.sieve_size >= 2 { false => 0, true => 1 };
        for num in (3..self.sieve_size).step_by(2) {
            if self.bits[num] {
                if show_results {
                    print!("{}, ", num);
                }
                count += 1;
            }
        }

        if show_results {
            println!("");
        }

        println!("Passes: {}, Time: {:.6}, Avg: {:.6}, Limit: {}, Count1: {}, Count2: {}, Valid: {}",
            passes,
            duration.as_secs_f64(),
            duration.as_secs_f64() / (passes as f64),
            self.sieve_size,
            count,
            self.count_primes(),
            self.validate_results());
    }

    fn count_primes(&self) -> usize {
        let mut count = match self.sieve_size >= 2 { false => 0, true => 1 };
        for i in (3..self.sieve_size).step_by(2) {
            if self.bits[i.try_into().unwrap()] {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    let mut passes = 0;
    let start_time = Instant::now();
    const FIVE_SECONDS: Duration = Duration::from_secs(5);
    
    loop {
        let mut seive = PrimeSieve::new(1_000_000);
        seive.run_sieve();
        passes += 1;
        if start_time.elapsed() >= FIVE_SECONDS
        {
            seive.print_results(false, start_time.elapsed(), passes);
            break;
        }
    }
}

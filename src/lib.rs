use std::sync::{Arc, Mutex};
use std::thread;

mod bits;
mod cpu;

// Check if a number is prime
pub fn is_prime(num: u64) -> bool {

  // If num is 2, then it is prime
  if num == 2 { return true }
  // If num is less than 2 or is even, then it isn't prime
  if num < 2 || num % 2 == 0 { return false }

  // For each odd number between 3 and the square root of num,
  // if it's a divisor of num then num isn't prime
  let mut i: u64 = 3;
  while i < (num as f64).sqrt() as u64 {
    
    if num % i == 0 {
      return false;
    }

    i += 2;
  }

  // Else num is prime
  true
}

// Split a number into its prime factors
pub fn split_into_factors(num: u64) -> Vec<u64> {

  // Duplicate num as mutable
  let mut num: u64 = num;
  // Create a vector to store the factors
  let mut factors: Vec<u64> = Vec::new();

  if num <= 1 { return factors }

  // First, while the number is divisible by two,
  // divide it by two and push 2 to the vector
  while num % 2 == 0 {
    factors.push(2);
    num /= 2;
  }

  // Then, for each odd number between 3 and the square root of the current num,
  // if it's a divisor of num divide num by that number and add the number to the vector
  let mut i: u64 = 3;
  let mut num_sqrt: u64 = (num as f64).sqrt() as u64;
  while i <= num_sqrt {

    while num % i == 0 {
      factors.push(i);
      num /= i;
      num_sqrt = (num as f64).sqrt() as u64;
    }

    i += 2;
  }

  // If num is a prime number add itself to the factors
  if num > 1 {
    factors.push(num);
  }

  // Return the vector
  factors
}

// Greatest Common Divisor
pub fn gcd(x: u64, y: u64) -> u64 {

  // If any of x or y is 0, return the other one
  if x == 0 || y == 0 { return x + y }
  // If the numbers are equal, return any of them
  if x == y { return x }

  // Declare x and y as mutable
  let (mut x, mut y) = (x, y);

  // While y is not 0; x = y, y = x mod y
  while y != 0 {
    (x, y) = (y, x % y);
  }
  
  // Return x
  x
}

// Least Common Multiple
pub fn lcm(x: u64, y: u64) -> u128 {
  if x == 0 && y == 0 { return 0 }
  x as u128 * ((y as u128) / (gcd(x, y) as u128))
}

// Initial sieve
fn simple_sieve(size: u32) -> Vec<u32> {

  // Create a vector to store all prime numbers found
  let mut primes: Vec<u32> = Vec::new();

  // Handle specific scenarios 
  if size >= 2 { primes.push(2) }
  if size <= 2 { return primes }

  // Create the sieve
  let mut sieve: Vec<u8> = vec![0xff; ((size + 13) / 16) as usize];
  // Set the last bits that doesn't have to be sieved to not prime
  if sieve.len() != 0 {
    bits::unset_last_bits(sieve.last_mut().unwrap(), (7 - ((size - 3) / 2) % 8) as u8);
  }

  for i in 0..sieve.len() {
    for bit in 0..8 {

      // If the bit corresponding to this number is unset,
      // it is composite, so continue.
      if bits::is_bit_unset(&sieve[i as usize], bit) { continue }

      // If it is prime, add it to the vector
      let i: u32 = (i*16) as u32 + (bit*2) as u32 + 3;
      primes.push(i);

      // If i to the power of 2 is greater than the last element,
      // i can't be divisor of any of the remaining numbers, so continue.
      if (i as u64).pow(2) > size as u64 { continue }

      // Unset the bit corresponding to all multiples of i
      let mut multiple: u32 = i.pow(2);
      while multiple <= size {
        bits::unset_bit(&mut sieve[((multiple - 3) / 16) as usize], (((multiple - 3) / 2) % 8) as u8);
        // Add 2 times i because even numbers are not on the sieve
        multiple += 2*i;
      }

    }
  }

  primes
}

// Sieve a segment
fn segment_sieve(sieve: &mut Vec<u8>, primes: &Vec<u32>, low: usize, high: usize) -> u32 {

  // Handle specific scenarios
  if low == 2 && high == 2 { return 1 }
  else if low > high { return 0 }
  else if low == high && low % 2 == 0 { return 0 }

  // Update low and high to be odd numbers
  let size: usize = std::cmp::max((high - low).div_ceil(16), 1);
  let low: usize = if low < 3 { 3 } else if low == high { low } else if low % 2 == 0 { low + 1 } else { low };
  let high: usize = if low == high { high } else if high % 2 == 0 { high - 1 } else { high };

  // Fill the sieve and reset the count
  assert!(size <= sieve.len());
  sieve.fill(0xff);
  let mut count: u32 = 0;

  // Set the last bits that doesn't have to be sieved to not prime
  bits::unset_last_bits(&mut sieve[size - 1], (7 - ((high - low) / 2) % 8) as u8);

  // For each prime (skipping the 2, which is not needed on odd numbers)
  for prime in primes.iter().skip(1) {
    // Get the first multiple
    let mut multiple: usize = low.div_ceil(*prime as usize) * (*prime as usize);
    if multiple % 2 == 0 { multiple += *prime as usize }
    
    // Mark all multiples as not primes
    while multiple <= high {
      bits::unset_bit(&mut sieve[(multiple - low) / 16], (((multiple - low) / 2) % 8) as u8);
      multiple += 2 * (*prime as usize);
    }
  }

  // Count how many primes are there in this segment
  for byte in sieve.iter().take(size) {
    count += bits::count_set_bits(&byte) as u32;
  }

  count
}

// Count the number of prime numbers below or equal to limit
pub fn count_primes(limit: usize, start: Option<usize>, threads: Option<usize>, cache: Option<usize>) -> usize {

  if limit < 2 { return 0 }

  let threads: usize = threads.unwrap_or(cpu::get_cores());
  let cache: usize = cache.unwrap_or(cpu::get_cache_size());
  let start: usize = std::cmp::max(start.unwrap_or(2), 2);
  
  let sqrt: u32 = (limit as f64).sqrt() as u32;
  let segment_size: usize = std::cmp::min(std::cmp::max(sqrt as usize, cache * 16), limit - std::cmp::max(sqrt as usize, start - 1)).div_ceil(16) * 16;
  let segment_sieve_size: usize = segment_size.div_ceil(16);
  
  let small_primes: Arc<Vec<u32>> = Arc::new(simple_sieve(sqrt));
  let count: Arc<Mutex<usize>> = Arc::new(Mutex::new(small_primes.len()));
  let iter: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

  if start > 2 {
    let mut num = count.lock().unwrap();
    *num -= if start > sqrt as usize {
      small_primes.len()
    }
    else {
      small_primes.iter().filter(|&&x| (x as usize) < start).count()
    };
  }

  let mut handles = vec![];

  for _ in 0..threads {

    let mut sieve: Vec<u8> = vec![0xff; segment_sieve_size];
    let count: Arc<Mutex<usize>> = Arc::clone(&count);
    let iter: Arc<Mutex<Option<u32>>> = Arc::clone(&iter);
    let small_primes: Arc<Vec<u32>> = Arc::clone(&small_primes);

    let handle = thread::spawn(move || {

      let mut low: usize;
      let mut high: usize;

      loop {
        {
          let mut iter = iter.lock().unwrap();
          if let Option::None = *iter {
            break;
          }

          low = start + (iter.unwrap() as usize * segment_size);
          high = std::cmp::min(low + segment_size - 1, limit);
          if high >= limit {
            *iter = None;
          }
          else {
            *iter = Some(iter.unwrap() + 1);
          }
        }

        let current_count = segment_sieve(&mut sieve, &small_primes, low, high) as usize;

        {
          let mut num = count.lock().unwrap();
          *num += current_count;
        }
      }
    });

    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  let num = count.lock().unwrap();
  num.clone()
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn test_is_prime() {
    assert_eq!(is_prime(0), false);
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(6), false);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(99), false);
    assert_eq!(is_prime(100), false);
    assert_eq!(is_prime(101), true);
    assert_eq!(is_prime(102), false);
    assert_eq!(is_prime(103), true);
    assert_eq!(is_prime(4_294_967_290), false);
    assert_eq!(is_prime(4_294_967_291), true);
    assert_eq!(is_prime(4_294_967_292), false);
    assert_eq!(is_prime(4_294_967_295), false);
    assert_eq!(is_prime(4_294_967_296), false);
    assert_eq!(is_prime(18_446_744_073_709_551_614), false);
    assert_eq!(is_prime(18_446_744_073_709_551_615), false);
  }

  #[test]
  fn test_factors() {
    assert_eq!(split_into_factors(0), vec![]);
    assert_eq!(split_into_factors(1), vec![]);
    assert_eq!(split_into_factors(2), vec![2]);
    assert_eq!(split_into_factors(3), vec![3]);
    assert_eq!(split_into_factors(4), vec![2, 2]);
    assert_eq!(split_into_factors(5), vec![5]);
    assert_eq!(split_into_factors(6), vec![2, 3]);
    assert_eq!(split_into_factors(7), vec![7]);
    assert_eq!(split_into_factors(99), vec![3, 3, 11]);
    assert_eq!(split_into_factors(100), vec![2, 2, 5, 5]);
    assert_eq!(split_into_factors(101), vec![101]);
    assert_eq!(split_into_factors(102), vec![2, 3, 17]);
    assert_eq!(split_into_factors(103), vec![103]);
    assert_eq!(split_into_factors(4_294_967_290), vec![2, 5, 19, 22_605_091]);
    assert_eq!(split_into_factors(4_294_967_291), vec![4_294_967_291]);
    assert_eq!(split_into_factors(4_294_967_292), vec![2, 2, 3, 3, 7, 11, 31, 151, 331]);
    assert_eq!(split_into_factors(4_294_967_295), vec![3, 5, 17, 257, 65_537]);
    assert_eq!(split_into_factors(4_294_967_296), vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]);
    assert_eq!(split_into_factors(18_446_744_073_709_551_614), vec![2, 7, 7, 73, 127, 337, 92_737, 649_657]);
    assert_eq!(split_into_factors(18_446_744_073_709_551_615), vec![3, 5, 17, 257, 641, 65_537, 6_700_417]);
  }

  #[test]
  fn test_gcd() {
    assert_eq!(gcd(0, 0), 0);
    assert_eq!(gcd(0, 1), 1);
    assert_eq!(gcd(1, 0), 1);
    assert_eq!(gcd(1, 1), 1);
    assert_eq!(gcd(2, 4), 2);
    assert_eq!(gcd(4, 2), 2);
    assert_eq!(gcd(3, 9), 3);
    assert_eq!(gcd(9, 3), 3);
    assert_eq!(gcd(6, 8), 2);
    assert_eq!(gcd(7, 13), 1);
    assert_eq!(gcd(99, 121), 11);
    assert_eq!(gcd(100, 80), 20);
    assert_eq!(gcd(101, 103), 1);
    assert_eq!(gcd(102, 170), 34);
    assert_eq!(gcd(103, 206), 103);
    assert_eq!(gcd(1_234_567_890, 987_654_321), 9);
    assert_eq!(gcd(4_294_967_295, 65_536), 1);
    assert_eq!(gcd(4_294_967_296, 65_536), 65_536);
    assert_eq!(gcd(4_294_967_290, 4_294_967_295), 5);
    assert_eq!(gcd(4_294_967_291, 4_294_967_292), 1);
    assert_eq!(gcd(4_294_967_292, 4_294_967_296), 4);
    assert_eq!(gcd(18_446_744_073_709_551_614, 18_446_744_073_709_551_615), 1);
    assert_eq!(gcd(18_446_744_073_709_551_615, 18_446_744_073_709_551_615), 18_446_744_073_709_551_615);
  }

  #[test]
  fn test_lcm() {
    assert_eq!(lcm(0, 0), 0);
    assert_eq!(lcm(0, 1), 0);
    assert_eq!(lcm(1, 0), 0);
    assert_eq!(lcm(1, 1), 1);
    assert_eq!(lcm(2, 4), 4);
    assert_eq!(lcm(4, 2), 4);
    assert_eq!(lcm(3, 9), 9);
    assert_eq!(lcm(6, 8), 24);
    assert_eq!(lcm(7, 13), 91);
    assert_eq!(lcm(99, 121), 1089);
    assert_eq!(lcm(100, 80), 400);
    assert_eq!(lcm(101, 103), 10403);
    assert_eq!(lcm(102, 170), 510);
    assert_eq!(lcm(103, 206), 206);
    assert_eq!(lcm(4_294_967_295, 65_536), 281_474_976_645_120);
    assert_eq!(lcm(4_294_967_296, 65_536), 4_294_967_296);
    assert_eq!(lcm(1_234_567_890, 987_654_321), 135_480_701_236_261_410);
    assert_eq!(lcm(4_294_967_290, 4_294_967_295), 3_689_348_808_728_956_110);
    assert_eq!(lcm(4_294_967_291, 4_294_967_292), 18_446_744_035_054_845_972);
    assert_eq!(lcm(4_294_967_292, 4_294_967_296), 4_611_686_014_132_420_608);
    assert_eq!(lcm(18_446_744_073_709_551_614, 18_446_744_073_709_551_615), 340_282_366_920_938_463_408_034_375_210_639_556_610);
  }

  #[test]
  fn test_simple_sieve() {
    assert_eq!(simple_sieve(0), vec![]);
    assert_eq!(simple_sieve(1), vec![]);
    assert_eq!(simple_sieve(2), vec![2]);
    assert_eq!(simple_sieve(3), vec![2, 3]);
    assert_eq!(simple_sieve(4), vec![2, 3]);
    assert_eq!(simple_sieve(10), vec![2, 3, 5, 7]);
    assert_eq!(simple_sieve(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    assert_eq!(simple_sieve(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    assert_eq!(simple_sieve(50), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]);
    assert_eq!(simple_sieve(100), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]);
    assert_eq!(simple_sieve(101), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101]);
    assert_eq!(simple_sieve(102), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101]);
    assert_eq!(simple_sieve(199), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199]);
    assert_eq!(simple_sieve(1_000), vec![
      2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
      101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199,
      211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317,
      331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443,
      449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577,
      587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
      709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839,
      853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997
    ]);
    assert_eq!(simple_sieve(10_000).len(), 1229);
    assert_eq!(simple_sieve(100_000).len(), 9592);
    assert_eq!(simple_sieve(1_000_000).len(), 78498);
    assert_eq!(simple_sieve(10_000_000).len(), 664579);
  }
  
  #[test]
  fn test_count_primes() {
    assert_eq!(count_primes(10, None, None, None), 4);
    assert_eq!(count_primes(10, Some(2), None, None), 4);
    assert_eq!(count_primes(10, Some(3), None, None), 3);
    assert_eq!(count_primes(10, Some(5), None, None), 2);
    assert_eq!(count_primes(10, Some(11), None, None), 0);
    assert_eq!(count_primes(100, None, None, None), 25);
    assert_eq!(count_primes(1000, None, None, None), 168);
    assert_eq!(count_primes(10_000, None, None, None), 1229);
    assert_eq!(count_primes(100_000, None, None, None), 9592);
    assert_eq!(count_primes(1_000_000, None, None, None), 78498);
    assert_eq!(count_primes(10_000_000, None, None, None), 664579);
    assert_eq!(count_primes(100, Some(50), None, None), 10);
    assert_eq!(count_primes(100, Some(97), None, None), 1);
    assert_eq!(count_primes(100, Some(98), None, None), 0);
    assert_eq!(count_primes(2, None, None, None), 1);
    assert_eq!(count_primes(1, None, None, None), 0);
    assert_eq!(count_primes(0, None, None, None), 0);
    // Test with explicit threads and cache
    assert_eq!(count_primes(100, None, Some(1), Some(1)), 25);
    assert_eq!(count_primes(100, None, Some(4), Some(2)), 25);
  }
}

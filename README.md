Primeutils are a collection of tools for working with prime numbers. The tools included are:
 - Counting prime numbers below a limit or between two numbers
 - Checking if a number is a prime number
 - Split a number into its prime factors
 - Finding the lowest common multiple of two numbers
 - Finding the greatest common divisor of two numbers

# License

This project is licensed under the GPL-3.0-or-later license.

This project uses the following Rust crates licensed under the MIT license:
 - [`num_cpus`](https://crates.io/crates/num_cpus) — used to determine the optimal number of threads for the sieve
 - [`raw-cpuid`](https://crates.io/crates/raw-cpuid) — used to determine the size of the L1 cache of the processor for the sieve

Also, the dependencies of the crates are licensed under the MIT license, too:
 - [`bitflags`](https://crates.io/crates/bitflags)
 - [`hermit-abi`](https://crates.io/crates/hermit-abi)
 - [`libc`](https://crates.io/crates/libc)

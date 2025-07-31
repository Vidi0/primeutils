use std::env;
use std::process;

// Count how many prime numbers are there below count_limit
struct Count {
  count_limit: usize,
  count_start: Option<usize>,
  threads: Option<usize>,
  cache: Option<usize>,
}

// Check if num is prime
struct IsPrime {
  num: u64,
}

struct Factors {
  num: u64,
}

struct GCD {
  x: u64,
  y: u64,
}

struct LCM {
  x: u64,
  y: u64,
}

enum Arguments {
  Help(),
  Count(Count),
  IsPrime(IsPrime),
  Factors(Factors),
  GCD(GCD),
  LCM(LCM),
}

fn parse_count(args: Vec<String>) -> Result<Count, String> {
  let mut count_limit: Option<usize> = None;
  let mut count_start: Option<usize> = None;
  let mut threads: Option<usize> = None;
  let mut cache: Option<usize> = None;
  
  let mut i: usize = 0;
  while i < args.len() {
    let arg: &String = &args[i];
  
    if arg == "-t" {
      if threads != None { return Err(String::from(r#"Value already set for the parameter "-t""#)) }
  
      let val: Option<&String> = args.get(i+1);
      if let None = val { return Err(String::from(r#"Missing value for option "-t""#)) }
  
      let val: Result<usize, std::num::ParseIntError> = val.unwrap().parse::<usize>();
      if let Err(_error) = val { return Err(String::from(r#"Invalid value for option "-t""#)) }
  
      threads = Some(val.unwrap());
      i += 1;
    }
    else if arg == "-s" {
      if cache != None { return Err(String::from(r#"Value already set for the parameter "-s""#)) }
  
      let val: Option<&String> = args.get(i+1);
      if let None = val { return Err(String::from(r#"Missing value for option "-s""#)) }
  
      let val: Result<usize, std::num::ParseIntError> = val.unwrap().parse::<usize>();
      if let Err(_error) = val { return Err(String::from(r#"Invalid value for option "-s""#)) }
  
      cache = Some(val.unwrap());
      i += 1;
    }
    else if arg.starts_with("-") {
      return Err(String::from(r#"Invalid option: ""#) + arg + &String::from(r#"""#));
    }
    else {
      if count_limit != None { return Err(String::from("Count limit already set!")); }

      if arg.contains("..") {
        let parts: Vec<&str> = arg.split("..").collect();

        if parts.len() > 2 {
          return Err(String::from(r#"Error while parsing count start and limit: more than one delimiter ".." found"#));
        }
        else if parts.len() < 2 {
          return Err(String::from("Error while parsing count start and limit: limit missing"));
        }
        else {
          let val = parts[0].parse::<usize>();
          if let Err(_error) = val { return Err(String::from("Error while parsing count start: invalid number")); }
          count_start = Some(val.unwrap());

          let val = parts[1].parse::<usize>();
          if let Err(_error) = val { return Err(String::from("Error while parsing count limit: invalid number")); }
          count_limit = Some(val.unwrap());

          if count_start > count_limit {
            return Err(String::from("Error while parsing count start and limit: start is higher than limit"));
          }
        }
      }
      else {
        let val = arg.parse::<usize>();
        if let Err(_error) = val { return Err(String::from("Error while parsing count limit: invalid number")); }
        count_limit = Some(val.unwrap());
      }
    }
  
    i += 1;
  }
  
  if let None = count_limit {
    return Err(String::from("The count limit should be specified!"));
  }
  
  Ok(Count {
    count_limit: count_limit.unwrap(),
    count_start,
    threads,
    cache,
  })
}

fn parse_is_prime(args: Vec<String>) -> Result<IsPrime, String> {
  let mut num: Option<u64> = None;

  let mut i: usize = 0;
  while i < args.len() {
    let arg: &String = &args[i];

    if num != None { return Err(String::from("Number to check already set!")); }
  
    let val = arg.parse::<u64>();
    if let Err(_error) = val { return Err(String::from("Error while parsing number to check: invalid number")); }

    num = Some(val.unwrap());

    i += 1;
  }

  if let None = num {
    return Err(String::from("The number to check should be specified!"));
  }
  
  Ok(IsPrime {
    num: num.unwrap(),
  })
}

fn parse_factors(args: Vec<String>) -> Result<Factors, String> {
  let mut num: Option<u64> = None;
  
  let mut i: usize = 0;
  while i < args.len() {
    let arg: &String = &args[i];

    if num != None { return Err(String::from("Number to split already set!")); }
    
    let val = arg.parse::<u64>();
    if let Err(_error) = val { return Err(String::from("Error while parsing number to split: invalid number")); }
  
    num = Some(val.unwrap());

    i += 1;
  }

  if let None = num {
    return Err(String::from("The number to split should be specified!"));
  }
    
  Ok(Factors {
    num: num.unwrap(),
  })
}

fn parse_gcd(args: Vec<String>) -> Result<GCD, String> {
  let mut x: Option<u64> = None;
  let mut y: Option<u64> = None;

  let mut i: usize = 0;
  while i < args.len() {
    let arg: &String = &args[i];

    if x != None && y != None { return Err(String::from("Numbers to compute already set!")); }
  
    let val = arg.parse::<u64>();
    if let Err(_error) = val { return Err(String::from("Error while parsing number to compute: invalid number")); }

    if let None = x {
      x = Some(val.unwrap());
    }
    else if let None = y {
      y = Some(val.unwrap());
    }

    i += 1;
  }

  if let None = x {
    return Err(String::from("The numbers to compute should be specified!"));
  }
  if let None = y {
    return Err(String::from("Two numbers to compute should be specified!"));
  }
  
  Ok(GCD {
    x: x.unwrap(),
    y: y.unwrap(),
  })
}

fn parse_lcm(args: Vec<String>) -> Result<LCM, String> {
  let mut x: Option<u64> = None;
  let mut y: Option<u64> = None;

  let mut i: usize = 0;
  while i < args.len() {
    let arg: &String = &args[i];

    if x != None && y != None { return Err(String::from("Numbers to compute already set!")); }
  
    let val = arg.parse::<u64>();
    if let Err(_error) = val { return Err(String::from("Error while parsing number to compute: invalid number")); }

    if let None = x {
      x = Some(val.unwrap());
    }
    else if let None = y {
      y = Some(val.unwrap());
    }

    i += 1;
  }

  if let None = x {
    return Err(String::from("The numbers to compute should be specified!"));
  }
  if let None = y {
    return Err(String::from("Two numbers to compute should be specified!"));
  }
  
  Ok(LCM {
    x: x.unwrap(),
    y: y.unwrap(),
  })
}

fn parse_arguments(mut args: Vec<String>) -> Result<Arguments, String> {
  
  if args.len() <= 0 {
    return Ok(Arguments::Help());
  }

  let command: &String = &args[0].clone();
  args.remove(0);
  
  // How help
  if command == "help" || command == "--help" || command == "-h" {
    Ok(Arguments::Help())
  }
  // Count primes below a limit
  else if command == "count" {
    parse_count(args).map(|argument| Arguments::Count(argument)).map_err(|err| String::from(r#"Command "count" arguments: "#) + &err)
  }
  // Check if a number is prime
  else if command == "is_prime" {
    parse_is_prime(args).map(|argument| Arguments::IsPrime(argument)).map_err(|err| String::from(r#"Command "is_prime" arguments: "#) + &err)
  }
  // Split a number into its prime factors
  else if command == "factors" {
    parse_factors(args).map(|argument| Arguments::Factors(argument)).map_err(|err| String::from(r#"Command "factors" arguments: "#) + &err)
  }
  // Split a number into its prime factors
  else if command == "gcd" {
    parse_gcd(args).map(|argument| Arguments::GCD(argument)).map_err(|err| String::from(r#"Command "gcd" arguments: "#) + &err)
  }
  // Split a number into its prime factors
  else if command == "lcm" {
    parse_lcm(args).map(|argument| Arguments::LCM(argument)).map_err(|err| String::from(r#"Command "lcm" arguments: "#) + &err)
  }
  // Invalid command
  else {
    Err(String::from("Command not found: \"") + command + String::from("\"").as_str())
  }
}

fn print_help() {
  println!("\
Usage: primes-rs COMMAND REQUIRED_OPTIONS [OPTIONAL_OPTIONS]
Commands and its options:
  help               Display this help.
  count              Count how many prime numbers there are between start and limit.
    [START]..LIMIT     Set the start and limit of the sieve (separated by \"..\").
    [-t NUM]           How many threads should be used to sieve.
    [-s NUM]           How much cache should be used to sieve.
  is_prime           Check if num is prime.
    NUM                 The num to check.
  factors            Split num into its prime factors.
    NUM                 The num to split.
  gcd                Get the greatest common divisor of two numbers.
    X                   One number.
    Y                   The other number.
  lcm                Get the least common multiple of two numbers.
    X                   One number.
    Y                   The first number.\
");
}

fn main() {

  let mut args: Vec<String> = env::args().collect();
  args.remove(0);
  let args: Arguments = parse_arguments(args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments:\n{err}");
    process::exit(1);
  });

  match args {
    Arguments::Help() => {
      print_help();
    },

    Arguments::Count(count) => {
      let primes: usize = primeutils::count_primes(count.count_limit, count.count_start, count.threads, count.cache);
         
      match count.count_start {
        None => {
          println!("There are {} prime numbers less than or equal to {}", primes, count.count_limit);
        },
        Some(start) => {
          println!("There are {} prime numbers between {} and {}", primes, start, count.count_limit);
        }
      }
    },

    Arguments::IsPrime(is) => {
      let is_prime: bool = primeutils::is_prime(is.num);

      if is_prime { println!("The number {} is prime", is.num) }
      else { println!("The number {} is not prime", is.num) }
    },

    Arguments::Factors(fac) => {
      let factors: Vec<u64> = primeutils::split_into_factors(fac.num);

      println!("The number {} can be split into {:?}", fac.num, factors);
    },

    Arguments::GCD(gcd) => {
      let greatest_common_divisor: u64 = primeutils::gcd(gcd.x, gcd.y);

      println!("The greatest common divisor of {} and {} is {}", gcd.x, gcd.y, greatest_common_divisor);
    },

    Arguments::LCM(lcm) => {
      let least_common_multiple: u128 = primeutils::lcm(lcm.x, lcm.y);

      println!("The least common multiple of {} and {} is {}", lcm.x, lcm.y, least_common_multiple);
    },
  }

}
struct FactoredNumber {
  number: u32,
  factors: Vec<(u32, u32)>
}

impl FactoredNumber {
  pub fn new(number: u32) -> FactoredNumber {
    FactoredNumber {
      number,
      factors: Vec::new()
    }
  }

  pub fn calculate_factors(&mut self) {
    if is_prime(self.number) == true {
      self.factors.push((self.number, 1));
      return;
    }
    let mut copied_number = self.number;
    while copied_number > 1 {
      let mut i = 2;
      while copied_number % i != 0 {
        i = get_next_prime(i);
      }
      if self.increase_power_if_factor_exists(i) == false {
        self.factors.push((i, 1));
      }
      copied_number /= i;
    }
  }

  pub fn increase_power_if_factor_exists(&mut self, factor: u32) -> bool {
    let mut result = false;
    for existing_factor in self.factors.iter_mut() {
      if existing_factor.0 == factor { 
        existing_factor.1 += 1;
        result = true;
        break;
      }
    }
    result
  }

  pub fn to_string(factors: Vec<(u32, u32)>) -> String {
    let mut result = String::new();
    for factors in factors.iter() {
      result.push_str(&(factors.0.to_string())[..]);
      result.push_str("^");
      result.push_str(&(factors.1.to_string())[..]);
      result.push_str(" ");
    }
    return result;
  }
}

pub fn is_prime(n: u32) -> bool {
  let root = (n as f64).sqrt();
  let root = math::round::floor(root, 0) as u32;
  let mut counter = 2;
  let mut flag = true;
  while counter <= root {
      if n % counter == 0 {
          flag = false;
          break;
      }
      counter += 1;
  }
  return flag;
}

pub fn get_next_prime(n: u32) -> u32 {
  let mut result = n + 1;
  while is_prime(result) == false {
    result += 1;
  }
  return result;
}

pub fn merge_factors(factor1: Vec<(u32, u32)>, factor2: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
  let factor1 = factor1.clone();
  let factor2 = factor2.clone();
  let mut merged_factors: Vec<(u32, u32)> = Vec::new();
  let mut i = 0;
  let mut j = 0;
  while i < factor1.len() && j < factor2.len() {
    if factor1[i].0 < factor2[j].0 {
      merged_factors.push(factor1[i]);
      i += 1;
    } else if factor1[i].0 > factor2[j].0 {
      merged_factors.push(factor2[j]);
      j += 1;
    } else {
      // both are equal, take the larger power
      if factor1[i].1 > factor2[j].1 {
        merged_factors.push(factor1[i]);
      } else {
        merged_factors.push(factor2[j]);
      }
      i += 1;
      j += 1;
    }
  }
  while i < factor1.len() {
    merged_factors.push(factor1[i]);
    i += 1;
  }
  while j < factor2.len() {
    merged_factors.push(factor2[j]);
    j += 1;
  }
  merged_factors
}

pub fn runner() {
  let mut number1 = FactoredNumber::new(4);
  number1.calculate_factors();

  let mut number2 = FactoredNumber::new(20);
  number2.calculate_factors();

  let merged_factors = merge_factors(number1.factors, number2.factors);
  println!("{}", FactoredNumber::to_string(merged_factors));
}
pub fn is_palindrome(mut number: u32) -> bool {
  let copy = number;
  let mut reversed_number = 0;
  while number != 0 {
    let lsd = number % 10;
    reversed_number = reversed_number * 10 + lsd;
    number = number / 10;
  }
  copy == reversed_number
}

pub fn get_largest_palindrome() -> (u32, u32) {
  let mut num1 = 9999;
  let mut num2 = 9999;
  while num1 > 0 {
    while num2 > 0 {
      if is_palindrome(num1 * num2) {
        return (num1, num2);
      }
      num2 -= 1;
    }
    num1 -= 1;
  }
  return (num1, num2);
}

pub fn runner() {
  let (a, b) = get_largest_palindrome();
  println!("{} {}", a, b);
}
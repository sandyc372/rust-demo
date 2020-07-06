use std::collections;

pub struct Node {
  data: u64,
  left: Option<Box<Node>>,
  right: Option<Box<Node>>,
}

pub fn search(head: Option<&Box<Node>>, data: u64) -> Option<&Box<Node>> {
  match head {
    Some(node) => {
      if node.data == data {
        Some(node)
      } else {
        if data < node.data {
          search(node.left.as_ref(), data)
        } else {
          search(node.right.as_ref(), data)
        }
      }
    }
    None => None,
  }
}

pub fn insert(head: &mut Option<&mut Box<Node>>, data: u64) -> Result<(), String> {
  match head {
    Some(ref mut node) => {
      if data <= node.data {
        match &mut node.left {
          None => {
            node.left = Some(Box::new(Node {
              data: data,
              left: None,
              right: None,
            }));
          }
          Some(left_node) => {
            return insert(&mut Some(left_node), data);
          }
        };
      } else {
        match &mut node.right {
          None => {
            node.right = Some(Box::new(Node {
              data: data,
              left: None,
              right: None,
            }));
          }
          Some(right_node) => {
            return insert(&mut Some(right_node), data);
          }
        };
      }
      Ok(())
    }
    None => Err(String::from("Empty tree")),
  }
}

pub fn factorial(n: u64, map: &mut collections::HashMap<u64, u64>) -> u64 {
  let result;
  match map.get(&n) {
    Some(data) => {
      result = *data;
    }
    None => {
      if n == 0 {
        result = 1;
      } else {
        result = n * factorial(n - 1, map);
      }
      map.insert(n, result);
    }
  };
  return result;
}

pub fn sum_of_factorials(mut n: u64, map: &mut collections::HashMap<u64, u64>) -> u64 {
  let mut result = 0;
  while n > 0 {
    let digit = n % 10;
    result += factorial(digit, map);
    n = n / 10;
  };
  return result;
}

pub fn calculate_digit_factorial_chain(n: u64, map: &mut collections::HashMap<u64, u64>) -> u64 {
  let mut head = Some(Box::new(Node {
    data: n,
    left: None,
    right: None
  }));
  let mut chain_length: u64 = 1;
  let mut new_number = sum_of_factorials(n, map);
  while let None = search(head.as_ref(), new_number) {
    insert(&mut head.as_mut(), new_number);
    // print!("{} ", new_number);
    chain_length += 1;
    new_number = sum_of_factorials(new_number, map);
  };
  chain_length
}

pub fn runner() {
  let mut number: u64 = 0;
  let mut map: collections::HashMap<u64, u64> = collections::HashMap::new();
  let mut result = 0;
  while number < 1_000_000 {
    if calculate_digit_factorial_chain(number, &mut map) == 60 {
      result += 1;
    };
    number += 1;
  }
  println!("{}", result);
  /* let mut node = Some(Box::new(Node {
    data: 10,
    left: Some(Box::new(Node {
      data: 5,
      left: None,
      right: None,
    })),
    right: Some(Box::new(Node {
      data: 15,
      left: None,
      right: None,
    })),
  }));

  insert(&mut node.as_mut(), 45);

  insert(&mut node.as_mut(), 645);

  let result = search(node, 645);

  match result {
    Some(node) => println!("found {}", node.data),
    None => println!("Nothing found"),
  }; */
  
  // println!("factorial of {} is {}", number, factorial(number, &mut map));
}

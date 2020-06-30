pub struct Node {
  data: u32,
  left: Option<Box<Node>>,
  right: Option<Box<Node>>,
}

pub fn search(head: Option<Box<Node>>, data: u32) -> Option<Box<Node>> {
  match head {
    Some(node) => {
      if node.data == data {
        Some(node)
      } else {
        if data < node.data {
          search(node.left, data)
        } else {
          search(node.right, data)
        }
      }
    }
    None => None,
  }
}

pub fn insert(bst: &mut Node) {}

pub fn runner() {
  let number = 100;
  let mut node = Some(Box::new(Node {
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

  let result = search(node, 15);

  match result {
    Some(node) => {
      println!("found {}", node.data)
    }
    None => {
      println!("Nothing found")
    }
  };
}

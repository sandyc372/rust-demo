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

pub fn insert(head: &mut Option<&mut Box<Node>>, data: u32) -> Result<(), String> {
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
          },
          Some(left_node) => {
            let left = &mut *left_node;
            return insert(&mut Some(left), data);
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
          },
          Some(right_node) => {
            let right = &mut *right_node;
            return insert(&mut Some(right), data);
          }
        };
      }
      Ok(())
    }
    None => Err(String::from("Empty tree")),
  }
}

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

  insert(&mut node.as_mut(), 45);

  let result = search(node, 45);

  match result {
    Some(node) => println!("found {}", node.data),
    None => println!("Nothing found"),
  };
}

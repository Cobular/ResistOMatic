#[derive(PartialEq, Eq, Debug, Default)]
pub struct DynamicTreeNode<T> {
  val: T,
  left: Option<Box<DynamicTreeNode<T>>>,
  right: Option<Box<DynamicTreeNode<T>>>
}

impl<T: PartialOrd + Copy> DynamicTreeNode<T> {
  pub fn set_left(&mut self, val: T) {
    let new_node: DynamicTreeNode<T> = DynamicTreeNode { val, left: None, right: None };
    self.left = Some(Box::new(new_node));
  }

  pub fn set_right(&mut self, val: T) {
    let new_node: DynamicTreeNode<T> = DynamicTreeNode { val, left: None, right: None };
    self.right = Some(Box::new(new_node));
  }

  pub fn traverse(&self, target_val: T) -> (T, Option<&DynamicTreeNode<T>>) {
    (self.val, if self.val > target_val {
      self.right.as_ref().map(|e| e.as_ref())
    } else {
      self.left.as_ref().map(|e| e.as_ref())
    })
  }

  pub fn get_left(&self) -> Option<&DynamicTreeNode<T>> {
    self.left.as_ref().map(|e| e.as_ref())
  }

  pub fn get_right(&self) -> Option<&DynamicTreeNode<T>> {
    self.left.as_ref().map(|e| e.as_ref())
  }
}

pub fn process_slice<T: Clone>(slice: &[T]) -> DynamicTreeNode<T> {
  let midpoint = slice.len()/2;
  let left_slice = &slice[0..midpoint];
  let right_slice = &slice[midpoint+1..slice.len()];

  DynamicTreeNode {
    val: slice[midpoint].clone(),
    left: if left_slice.is_empty() {
      None
    } else {
      Some(Box::new(process_slice(left_slice)))
    },
    right: if right_slice.is_empty() {
      None
    } else {
      Some(Box::new(process_slice(right_slice)))
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_basic() {
    let input = [1, 2, 3];

    let tree = process_slice(&input);

    let mut goal = DynamicTreeNode {
      val: 2,
      ..Default::default()
    };

    goal.set_left(1);
    goal.set_right(3);

    assert_eq!(tree, goal)
  }

  #[test]
  fn test_small() {
    let input = [1, 2];

    let tree = process_slice(&input);

    let mut goal = DynamicTreeNode {
      val: 2,
      ..Default::default()
    };

    goal.set_left(1);

    assert_eq!(tree, goal)
  }

  #[test]
  fn test_larger() {
    let input = [1, 2, 3, 4];

    let tree = process_slice(&input);

    let mut goal = DynamicTreeNode {
      val: 3,
      ..Default::default()
    };

    goal.set_left(2);
    goal.set_right(4);

    goal.left.as_mut().unwrap().set_left(1);

    assert_eq!(tree, goal)
  }
}
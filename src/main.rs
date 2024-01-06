pub trait List<T: Copy> {
    fn get(&self, index: u32) -> Result<T, &str>;
    fn set(&mut self, index: u32, value: T) -> Result<T, &str>;
    fn add(&mut self, index: u32, value: T) -> Result<&Self, &str>;
    fn remove(&mut self, index: u32) -> Result<T, &str>;
    fn size(&self) -> u32;
}

struct LinkedListNode<T> {
    next: Option<Box<LinkedListNode<T>>>,
    value: T,
}
pub struct LinkedList<T> {
    head: Option<Box<LinkedListNode<T>>>,
}

impl<T: Copy> List<T> for LinkedList<T> {

    fn size(&self) -> u32 {
        let mut count = 0;
        let mut current: &Option<Box<LinkedListNode<T>>> = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }

    fn get(&self, index: u32) -> Result<T, &str> {
        let mut current: &Option<Box<LinkedListNode<T>>> = &self.head;
        for _ in 0..index {
            if let Some(node) = current {
                current = &node.next;
            } else {
                return Err("Index out of bounds");
            }
        }
        if let Some(node) = current {
            return Ok(node.value);
        } else {
            return Err("Index out of bounds");
        }
    }

    fn set(&mut self, index: u32, value: T) -> Result<T, &str> {
        let mut current = &mut self.head;
        for _ in 0..index {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                return Err("Index out of bounds");
            }
        }
        if let Some(node) = current {
            let result = Ok(node.value);
            node.value = value;
            return result;
        } else {
            return Err("Index out of bounds");
        }
    }

    fn add(&mut self, index: u32, value: T) -> Result<&Self, &str> {
        if (index == 0) {
            self.head = Some(Box::new(
                LinkedListNode {next: self.head.take(), value: value}
            ));
            return Ok(self);
        }
        let mut current = &mut self.head;
        for _ in 1..index {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                return Err("Index out of bounds");
            }
        }
        if let Some(node) = current {
            let next = node.next.take();
            node.next = Some(Box::new(
                LinkedListNode {next: next, value: value}
            ));
            return Ok(self);
        } else {
            return Err("Index out of bounds");
        }
    }

    fn remove(&mut self, index: u32) -> Result<T, &str> {
        return Err("Not implemented");
    }
}

#[cfg(test)]
mod tests {

    use rand::Rng;

    use super::List;
    use super::LinkedList;

    fn new_list() -> impl List<i32> {
        LinkedList {head: None}
    }

    #[test]
    fn test_list_size() {
        let list = new_list();
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_list_get_empty() {
        let list = new_list();
        assert_eq!(list.get(0), Err("Index out of bounds"));
        assert_eq!(list.get(1), Err("Index out of bounds"));
    }
    
    #[test]
    fn test_list_set_empty() {
        let mut rng = rand::thread_rng();
        let mut list = new_list();
        assert_eq!(list.set(0, rng.gen()), Err("Index out of bounds"));
        assert_eq!(list.set(1, rng.gen()), Err("Index out of bounds"));
    }

    #[test]
    fn test_list_add_empty() {
        let mut rng = rand::thread_rng();
        let mut list = new_list();
        for i in 1..3 {
            let result = list.add(i, rng.gen());
            assert!(result.is_err_and(|msg| msg == "Index out of bounds"));
        }
    }

    #[test]
    fn test_list_one_item() {
        let mut rng = rand::thread_rng();
        let value0: i32 = rng.gen();

        let mut list = new_list();
        assert!(list.add(0, value0).is_ok());
        assert_eq!(list.size(), 1);
        assert_eq!(list.get(0), Ok(value0));

        let new_value0: i32 = rng.gen();
        assert_eq!(list.set(0, new_value0), Ok(value0));
        assert_eq!(list.size(), 1);
        assert_eq!(list.get(0), Ok(new_value0));

        assert_eq!(list.get(1), Err("Index out of bounds"));
        assert_eq!(list.get(2), Err("Index out of bounds"));
    }

    #[test]
    fn test_list_two_items() {
        let mut rng = rand::thread_rng();
        let value0: i32 = rng.gen();
        let value1: i32 = rng.gen();
        
        let mut list = new_list();
        assert!(list.add(0, value1).is_ok());
        assert!(list.add(0, value0).is_ok());
        assert_eq!(list.size(), 2);
        assert_eq!(list.get(0), Ok(value0));
        assert_eq!(list.get(1), Ok(value1));

        let new_value0: i32 = rng.gen();
        let new_value1: i32 = rng.gen();
        assert_eq!(list.set(0, new_value0), Ok(value0));
        assert_eq!(list.set(1, new_value1), Ok(value1));
        assert_eq!(list.size(), 2);
        assert_eq!(list.get(0), Ok(new_value0));
        assert_eq!(list.get(1), Ok(new_value1));

        assert_eq!(list.get(2), Err("Index out of bounds"));
        assert_eq!(list.get(3), Err("Index out of bounds"));
    }

    // #[test]
    // fn test_list_add() {
    //     let mut list = new_list();

    //     list.add(0, 1);
    //     list.add(1, 2);
    //     list.add(2, 3);

    //     assert_eq!(list.size(), 3);
    // }

    // #[test]
    // fn test_list_remove() {
    //     let mut list = new_list();
    //     list.add(0, 1);
    //     list.add(1, 2);
    //     list.add(2, 3);

    //     assert_eq!(list.remove(1), Some(2));
    //     assert_eq!(list.size(), 2);
    // }

}

fn main() {}

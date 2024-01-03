trait List<T> {
    fn get(&self, index: u32) -> Result<&T, &str>;
    //fn set(&self, index: u32, value: T) -> Result<T>;
    //fn insert(&mut self, index: u32, value: T);
    //fn remove(&mut self, index: u32) -> Result<T>;
    fn size(&self) -> u32;
}

struct LinkedListNode<T> {
    next: Option<Box<LinkedListNode<T>>>,
    value: T,
}
struct LinkedList<T> {
    head: Option<Box<LinkedListNode<T>>>,
}

impl<T> List<T> for LinkedList<T> {

    fn get(&self, index: u32) -> Result<&T, &str> {
        let mut current: &Option<Box<LinkedListNode<T>>> = &self.head;
        for _ in 0..index {
            if let Some(node) = current {
                current = &node.next;
            } else {
                return Err("Index out of bounds");
            }
        }
        if let Some(node) = current {
            return Ok(&node.value);
        } else {
            return Err("Index out of bounds");
        }
    }

    fn size(&self) -> u32 {
        let mut count = 0;
        let mut current: &Option<Box<LinkedListNode<T>>> = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }
}

fn main() {
    let list: LinkedList<i32> = LinkedList {head: None};
    println!("size: {}", list.size());
}

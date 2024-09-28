// Title: Design Circular Deque
// URL: https://leetcode.com/problems/design-circular-deque/
// Difficulty: Medium

#![allow(dead_code)]

struct MyCircularDeque {
    queue: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            queue: Vec::with_capacity(k as usize),
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.queue.insert(0, value);
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.queue.push(value);
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.queue.is_empty() {
            return false;
        }
        self.queue.remove(0);
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.queue.is_empty() {
            return false;
        }
        self.queue.pop();
        true
    }

    fn get_front(&self) -> i32 {
        if self.queue.is_empty() {
            return -1;
        }
        self.queue[0]
    }

    fn get_rear(&self) -> i32 {
        if self.queue.is_empty() {
            return -1;
        }
        self.queue[self.queue.len() - 1]
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn is_full(&self) -> bool {
        self.queue.len() == self.queue.capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = MyCircularDeque::new(3);
        assert!(obj.insert_last(1));
        assert!(obj.insert_last(2));
        assert!(obj.insert_front(3));
        assert!(!obj.insert_front(4));
        assert_eq!(obj.get_rear(), 2);
        assert!(obj.is_full());
        assert!(obj.delete_last());
        assert!(obj.insert_front(4));
        assert_eq!(obj.get_front(), 4);
    }
}

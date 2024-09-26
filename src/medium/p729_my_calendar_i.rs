// Title: My Calendar I
// URL: https://leetcode.com/problems/my-calendar-i/
// Difficulty: Medium

#![allow(dead_code)]

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Event {
    start: i32,
    end: i32,
}

impl Event {
    fn new(start: i32, end: i32) -> Self {
        Event { start, end }
    }
}

struct MyCalendar {
    events: Vec<Event>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        MyCalendar { events: Vec::new() }
    }

    // O(nlogn) time complexity for inserting n events
    // O(n) space complexity for storing n events
    fn book(&mut self, start: i32, end: i32) -> bool {
        // search for the position to insert the new event
        let event = Event::new(start, end);
        let pos = match self.events.binary_search(&event) {
            Ok(_) => return false,
            Err(pos) => pos,
        };

        // check if the new event overlaps with the previous event
        if pos > 0 && start < self.events[pos - 1].end {
            return false;
        }

        // check if the new event overlaps with the next event
        if pos < self.events.len() && end > self.events[pos].start {
            return false;
        }

        // insert the new event
        self.events.insert(pos, event);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book() {
        let mut my_calendar = MyCalendar::new();
        assert!(my_calendar.book(10, 20));
        assert!(!my_calendar.book(15, 25));
        assert!(my_calendar.book(20, 30));
    }
}

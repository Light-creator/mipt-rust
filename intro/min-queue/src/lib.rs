#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    q: VecDeque<T>,
    mq: VecDeque<T>
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self { 
            q: VecDeque::<T>::new(),
            mq: VecDeque::<T>::new()
        }
    }

    pub fn push(&mut self, val: T) {
        if self.q.is_empty() {
            self.mq.push_back(val.clone());
        } else {
            loop {
                match self.mq.back() {
                    Some(i) => {
                        if *i > val {
                            self.mq.pop_back();
                        } else {
                            break;
                        }
                    },
                    _ => break,
                }
            }
            self.mq.push_back(val.clone());
        }
        self.q.push_back(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.q.front() == self.mq.front() {
            self.mq.pop_front();
        }
        self.q.pop_front()
    }

    pub fn front(&self) -> Option<&T> {
        self.q.front()
    }

    pub fn min(&self) -> Option<&T> {
        self.mq.front()
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }

    pub fn is_empty(&self) -> bool {
        self.q.is_empty()
    }
}

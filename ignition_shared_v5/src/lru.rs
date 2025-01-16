use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

const MAX_LRU_CACHE_SIZE: usize = 100;

#[derive(Debug)]
struct Node {
    key: String,
    prev: Option<Arc<Mutex<Node>>>,
    next: Option<Arc<Mutex<Node>>>,
}

struct DoublyLinkedList {
    head: Option<Arc<Mutex<Node>>>,
    tail: Option<Arc<Mutex<Node>>>,
    size: usize,
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn push_back(&mut self, key: String) -> Arc<Mutex<Node>> {
        let new_node = Arc::new(Mutex::new(Node {
            key,
            prev: None,
            next: None,
        }));

        // attach new_node to the tail of doubly linked list !!

        if let Some(tail) = &self.tail {
            tail.lock().unwrap().next = Some(new_node.clone());
            new_node.lock().unwrap().prev = Some(tail.clone());
        } 
        else {
            self.head = Some(new_node.clone());
        }

        self.tail = Some(new_node.clone());
        self.size += 1;

        new_node
    }

    fn remove_node(&mut self, node: Arc<Mutex<Node>>) {

        let node = node.lock().unwrap();

        if let Some(prev) = &node.prev {
            prev.lock().unwrap().next = node.next.clone();
        } 
        else {
            self.head = node.next.clone();
        }

        if let Some(next) = &node.next {
            next.lock().unwrap().prev = node.prev.clone();
        } 
        else {
            self.tail = node.prev.clone();
        }

        self.size -= 1;
    }

    fn pop_front(&mut self) -> Option<String> {
        self.head.clone().map(|head_node| {
            let key = head_node.lock().unwrap().key.clone();
            self.remove_node(head_node);
            key
        })
    }
}

pub struct LruCache {
    list: DoublyLinkedList,
    map: HashMap<String, Arc<Mutex<Node>>>,
}

impl LruCache {
    pub fn new() -> Self {
        Self {
            list: DoublyLinkedList::new(),
            map: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, key: String) {
        if self.map.contains_key(&key) {
            self.access_user(&key); // Move to the tail if it already exists
        } 
        else {
            let node = self.list.push_back(key.clone());
            self.map.insert(key, node);

            if self.list.size > MAX_LRU_CACHE_SIZE {
                if let Some(evicted_key) = self.list.pop_front() {
                    self.map.remove(&evicted_key);
                }
            }
        }
    }

    // !TODO! LRU CURRENTLY WORKS BY PUSHING THE USED ELEMENT AT THE END
    // !TODO! IT SHOULD ACTUALLY SWAP WITH ITS PRE-DECSSOR ELEMENT !!

    pub fn access_user(&mut self, key: &str) {
        if let Some(node) = self.map.get(key) {
            self.list.remove_node(node.clone());
            let new_node = self.list.push_back(key.to_string());
            self.map.insert(key.to_string(), new_node);
        }
    }

    pub fn remove_user(&mut self, key: &str) {
        if let Some(node) = self.map.remove(key) {
            self.list.remove_node(node);
        }
    }

    pub fn clear(&mut self) {
        self.list = DoublyLinkedList::new();
        self.map.clear();
    }
}


use std::collections::{HashMap, HashSet};

pub struct Node<K, V> {
    id: K,
    value: V,
    next: HashSet<K>,
}

impl<K, V> Node<K, V>
where
    K: Clone+std::cmp::Eq+std::hash::Hash
{
    pub fn new(id: K, value: V) -> Node<K, V> {
        Node{
            id: id,
            value: value,
            next: HashSet::new(),
        }
    }

    pub fn id(&self) -> K {
        self.id.clone()
    }

    pub fn has(&self, key: &K) -> bool {
        self.next.contains(key)
    }
}

pub struct Trie<K, V> {
    nodes: HashMap<K, Node<K, V>>,
    roots: HashSet<K>,
}

impl<K, V> Trie<K, V>
where
    K: Clone+std::cmp::Eq+std::hash::Hash
{
    pub fn new() -> Trie<K, V> {
        Trie{
            nodes: HashMap::new(),
            roots: HashSet::new(),
        }
    }

    pub fn insert(&mut self, mut word: Vec<(K, V)>) {
        let mut i = 0;
        let mut hashptr = &mut self.roots;
        while i < word.len() {
            let (key, value) = word.swap_remove(i);
            let node = Node::new(key, value);
            i += 1;
        }
        while word.len() > 0 {
            if let Some((key, value)) = word.pop() {
                //
            }
        }
    }
}

use std::{ops::Add, rc::Rc};

#[derive(Debug, Clone)]
pub struct Node<T>{
    value: T,
    left: Subtree<T>,
    right: Subtree<T>
}

pub enum Traversed<'a, T, G>{
    Node(&'a Subtree<T>),
    Value(G)
}

#[derive(Debug, Clone)]
pub struct Subtree<T>(pub Option<Rc<Node<T>>>);

impl<G: Ord + Copy + std::ops::Add<G, Output = G>> Add for Node<(Option<u8>, G)>{
    type Output = Node<(Option<u8>, G)>;
    fn add(self, rhs: Node<(Option<u8>, G)>) -> Self::Output {
        Self::Output{
            value: (None, self.value.1 + rhs.value.1),
            left: Subtree(Some(Rc::new(self))),
            right: Subtree(Some(Rc::new(rhs))),
        }
    }
}
impl<G> Node<(Option<u8>, G)> {
    pub fn new_leaf(number: u8, power: G) -> Self{
        let value = (Some(number), power);
        Self{
            value,
            left: Subtree(None),
            right: Subtree(None)
        }
    }
    pub fn find_u8(&self, previous_path: &Vec<bool>, collector: &mut [Vec<bool>; 256]){
        if let Some(val) = &self.value.0{
            collector[*val as usize] = previous_path.clone();
        }
        if let Some(left) = &self.left.0{
            let mut left_path = previous_path.clone();
            left_path.push(false);
            left.find_u8(&left_path, collector);
        }
        if let Some(right) = &self.right.0{
            let mut right_path = previous_path.clone();
            right_path.push(true);
            right.find_u8(&right_path, collector);
        }
    }
    pub fn traverse(&self, input: bool) -> Option<Traversed<(Option<u8>, G), u8>>{
        match self.value.0{
            Some(value) => {
                Some(Traversed::Value(value))
            },
            None => {
                match input{
                    true => {
                        Some(Traversed::Node(&self.right))
                    },
                    false => {
                        Some(Traversed::Node(&self.left))
                    },
                }
            },
        }
    }
}

pub fn vec_of_ut<T>(input: [T; 256]) -> Vec<(usize, T)>
where T: Ord + Default{
    let mut vect:Vec<_> = input.into_iter().enumerate().filter(|(_, t)|{
        *t != T::default()
    }).collect();
    vect.sort_by(|(_, a), (_, b)|{
        b.cmp(a)
    });
    vect
}


pub fn tree_from_vec<G>(input: Vec<(usize, G)>) -> BinaryTree<(Option<u8>, G)>
where G: Ord + Default + Copy + std::ops::Add<G, Output = G>{
    let mut nodes: Vec<_> = input.into_iter().map(|(number, power)|{
        Node::new_leaf(number as u8, power)
    }).collect();
    while nodes.len() > 1{
        if let (Some(leaf1), Some(leaf2)) = (nodes.pop(), nodes.pop()){
            nodes.push(leaf1 + leaf2);
        }
        nodes.sort_by(|node1, node2|{
            node2.value.1.cmp(&node1.value.1)
        });
    }
    return BinaryTree{root: Some(Rc::new(nodes.pop().unwrap()))};
}

pub struct BinaryTree<T>{
    pub root: Option<Rc<Node<T>>>,
}


impl<G> BinaryTree<(Option<u8>, G)>{
    pub fn make_byte_conversion_array(&self) -> [Vec<bool>; 256] {
        const EMPTY: Vec<bool> = vec![];
        let mut result = [EMPTY; 256];
        if self.root.is_none(){
            return result;
        }
        if let Some(root) = &self.root{
            if let Some(_) = root.value.0{
                println!("Why does root have value???");
            }
            if let Some(left) = &root.left.0{
                left.find_u8(&vec![false], &mut result)
            }


            if let Some(right) = &root.right.0{
                right.find_u8(&vec![true], &mut result)
            }
        }
        
        result
    }
}

#[cfg(feature = "draw")]
use std::collections::VecDeque;

#[cfg(feature = "draw")]
impl<T: std::fmt::Debug> BinaryTree<T> {
    pub fn draw(&self) {
        if let Some(root) = &self.root {
            let mut queue = VecDeque::new();
            queue.push_back(root);
            let mut level = 0;

            while !queue.is_empty() {
                let level_size = queue.len();
                let mut output = String::new();

                for _ in 0..level_size {
                    if let Some(node) = queue.pop_front() {
                        output.push_str(&format!("{:?} ", node.value));
                        if let Some(left) = &node.left.0 {
                            queue.push_back(left);
                        }
                        if let Some(right) = &node.right.0 {
                            queue.push_back(right);
                        }
                    }
                }
                println!("Level {}: {}", level, output);
                level += 1;
            }
        } else {
            println!("The tree is empty.");
        }
    }
}
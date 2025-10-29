use std::ops::Add;

#[derive(Debug)]
pub struct Node<T>{
    value: T,
    left: Subtree<T>,
    right: Subtree<T>
}

#[derive(Debug)]
struct Subtree<T>(Option<Box<Node<T>>>);

impl<G: Ord + Copy + std::ops::Add<G, Output = G>> Add for Node<(Option<u8>, G)>{
    type Output = Node<(Option<u8>, G)>;
    fn add(self, rhs: Node<(Option<u8>, G)>) -> Self::Output {
        Self::Output{
            value: (None, self.value.1 + rhs.value.1),
            left: Subtree(Some(Box::new(self))),
            right: Subtree(Some(Box::new(rhs))),
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
    return BinaryTree{root: Some(Box::new(nodes.pop().unwrap()))};
}

pub struct BinaryTree<T>{
    root: Option<Box<Node<T>>>,
}



use std::collections::VecDeque;

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
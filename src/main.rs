use std::{collections::HashMap};

#[derive(Debug)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    val: Vec<char>,
    freq: i32
}

const TEXT: &str = "Hello, world!";

fn main() {
    println!("{}", TEXT);

    let mut occurence = HashMap::new();

    // count occurences of chars in given text
    for char in TEXT.chars() {
        occurence.entry(char).and_modify(|val| *val += 1).or_insert(1);
    }
    
    // initialize the queue and fill up with all char nodes
    let mut queue = Vec::new();
    for (char, freq) in occurence.iter() {
        queue.push(Node { left: None, right: None, val: vec![*char], freq: *freq });
    }
    
    // reduce the queue to a tree until one node is left
    while queue.len() > 1 {
        queue.sort_by(|a, b| b.freq.cmp(&a.freq));

        let mut left = None;
        let mut right = None;
        let mut val = Vec::new();
        let mut freq = 0;

        if let Some(node) = queue.pop() {
            let mut left_val = node.val.clone();
            val.append(&mut left_val);
            freq += node.freq;
            left = Some(Box::new(node));
        }

        if let Some(node) = queue.pop() {
            let mut right_val = node.val.clone();
            val.append(&mut right_val);
            freq += node.freq;
            right = Some(Box::new(node));
        }

        queue.push(Node { left, right, val, freq});
    }

    // this is our root node
    let root = &queue[0];

    // generate codebook with the text and tree
    let mut codebook = HashMap::new();
    for char in TEXT.chars() {
        codebook.insert(char, encode(char, root));
    }

    // encode the text with the codebook
    let mut encoded_text = Vec::new();
    for char in TEXT.chars() {
        encoded_text.push(codebook.get(&char).unwrap());
    }

    // decode with codebook and convert to String
    let mut decoded_text = Vec::new();
    for encoded_char in &encoded_text {
        for (char, code) in &codebook {
            if encoded_char == &code {
                decoded_text.push(*char);
            }
        }
    }
    let decoded_text: String = decoded_text.iter().collect();

    println!("{:#?}", decoded_text);
    
}


fn encode(char: char, root: &Node) -> Vec<bool>{
    let mut code_word = Vec::new();

    if let Some(left) = &root.left {
        if left.val.contains(&char) {
            code_word.push(false);
            code_word.append(&mut encode(char, &left));
        }
    }

    if let Some(right) = &root.right {
        if right.val.contains(&char) {
            code_word.push(true);
            code_word.append(&mut encode(char, &right));
        }
    }

    code_word
}
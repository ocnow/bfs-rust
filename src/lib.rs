use std::collections::VecDeque;

struct Node{
    data : i32,
    left : Option<Box<Node>>,
    right : Option<Box<Node>>,
}

pub struct Tree{
    root : Option<Node>,
}

impl Tree{
    pub fn new() -> Tree{
        Tree { 
            root: None
        }
    }

    pub fn insert(&mut self, a_node_data : i32){
        if self.root.is_none(){
            self.root = Some(Node { data: a_node_data, left: None, right: None });
            return;
        }

        let mut current_root = self.root.as_mut().unwrap();
        loop{
            if a_node_data > current_root.data {
                //check if the right is none and if it is, move the a_node to right
                if current_root.right.is_none(){    
                    current_root.right = Some(Box::new(Node { data: a_node_data, left: None, right: None }));
                    break
                }
                //check if the right is some and if it is, move the right to current_root
                else{
                    current_root = current_root.right.as_mut().unwrap();
                }
            }

            if a_node_data < current_root.data {
                //check if the left is none and if it is, move the a_node to left
                if current_root.left.is_none(){    
                    current_root.left = Some(Box::new(Node { data: a_node_data, left: None, right: None }));                    break
                }
                //check if the left is some and if it is, move the left to current_root
                else{
                    current_root = current_root.left.as_mut().unwrap();
                }
            }
        }
    }

    pub fn bfs_string(&self) -> String{
        let mut ret_string = String::new();

        if let Some(root) = self.root.as_ref(){
            let mut bfs_vector : VecDeque<&Node> = VecDeque::new();
            
            bfs_vector.push_back(&root);
            while let Some(x) = bfs_vector.pop_front(){
                ret_string.push_str(&format!("{} ",x.data));
                if x.left.is_some(){
                    bfs_vector.push_back(x.left.as_ref().unwrap());
                    // println!("left:{}",x.left.as_ref().unwrap().data);
                } 
                if x.right.is_some(){
                    bfs_vector.push_back(x.right.as_ref().unwrap());
                    // println!("right:{}",x.right.as_ref().unwrap().data);
                } 
            }

            return ret_string.trim().to_string();
        }else{
            return String::from("empty tree"); 
        }
    }
}


#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn test_1(){
        let mut tree_instnance = Tree::new();

        tree_instnance.insert(12);
        tree_instnance.insert(10);
        tree_instnance.insert(13);
        tree_instnance.insert(27);
        tree_instnance.insert(59);
        tree_instnance.insert(0);
        tree_instnance.insert(-9);

        assert_eq!(tree_instnance.bfs_string(),"12 10 13 0 27 -9 59");
    }

    #[test]
    fn test_2(){
        let tree_instnance = Tree::new();

        assert_eq!(tree_instnance.bfs_string(),"empty tree");
    }
}

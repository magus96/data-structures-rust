#[derive(Debug)]
struct Node<T>{
    val: Option<(T, Box<Node<T>>)>

}


impl<T> Node<T>{

//Check for empty list
    fn empty() -> Self{
        
        Self{
            val: None
        }
    }

//Find length of linked list
    fn len(&self) -> usize{
        
        if let Some((_, next)) = self.val.as_ref(){
            return 1 + next.len();
        } else{
            return 0;
        }
    }

//Append to linked list
    fn append(&mut self, val: T){
        
        if let Some((_, next)) = &mut self.val{
            next.append(val);
        } else{
            self.val = Some((val, Box::new(Self::empty())));
        }
    }

//Remove element from list
    fn remove(&mut self) -> T{
        
        let mut removed_node:Option<(T, Box<Node<T>>)> = None;
        std::mem::swap(&mut self.val, &mut removed_node);
        let (val, mut node) = removed_node.unwrap();
        std::mem::swap(&mut self.val, &mut node.val);
        return val;
    }
}

//main function
fn main() {
    
    let mut h:Node<u8> = Node::empty();
    h.append(0);
    h.append(18);
    h.append(7);

    println!("Linked List: {:?}", h);

    dbg!(h.val.as_mut().unwrap().1.remove());
    dbg!(&h);
}

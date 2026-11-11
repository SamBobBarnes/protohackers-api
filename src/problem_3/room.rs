use std::collections::HashMap;

pub struct ChatRoom{
    store: HashMap<usize, String>,
}

impl ChatRoom{
    pub fn new() -> ChatRoom{
        ChatRoom{ store: Default::default()}
    }
    pub fn join(self,name:String){
        
    }
}
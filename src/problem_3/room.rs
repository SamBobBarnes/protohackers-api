use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use tokio::sync::{Mutex, MutexGuard};

pub struct ChatRoom{
    users: Mutex<HashMap<Token, String>>,
    queues: Mutex<HashMap<Token, Mutex<Vec<String>>>>
}

impl ChatRoom{
    pub fn new() -> ChatRoom{
        ChatRoom{ users: Mutex::new(HashMap::new()), queues: Mutex::new(HashMap::new())}
    }

    async fn get_queues(&self) -> MutexGuard<'_,HashMap<Token, Mutex<Vec<String>>>>{
        self.queues.lock().await
    }

    async fn get_users(&self) -> MutexGuard<'_,HashMap<Token, String>>{
        self.users.lock().await
    }

    pub async fn join(&self,name:String)-> Token{
        let id = Token::new(name.clone());
        let mut users = self.get_users().await;
        for (id,name) in users.iter(){
            self.queue_msg(id,&format!("* {} has joined the room\n",name)).await;
        }
        users.insert(id.clone(), name);
        id
    }

    pub async fn leave(&self,id:&Token){
        let mut users = self.get_users().await;
        let name = users.remove(id);
        for (id,_) in users.iter(){
            self.queue_msg(id,&format!("* {} has left the room\n",name.clone().unwrap())).await;
        }
    }

    async fn queue_msg(&self,id:&Token,msg:&str){
        self.queues.lock().await.get(id).unwrap().lock().await.push(msg.to_string());
    }

    pub async fn pop_msg(&self, id:&Token) -> Option<String>{
        let mut queues = self.get_queues().await;
        let mut queue = queues.get_mut(id).unwrap().lock().await;
        if queue.is_empty(){
            return None;
        }
        queue.pop()
    }
}

#[derive(Clone,Copy,Hash,Eq,PartialEq)]
pub struct Token {
    id: u64,
}

impl Token{
    fn new(hashable_str: String) -> Token{
        let mut hasher = DefaultHasher::new();
        hashable_str.hash(&mut hasher);
        let hash_result: u64 = hasher.finish();
        Token { id: hash_result }
    }
}
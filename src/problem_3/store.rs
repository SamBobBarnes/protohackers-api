use std::collections::HashMap;

pub struct Persistence{
    pub data: HashMap<String, String>
}

impl Persistence{
    pub fn new() -> Persistence{
        Persistence{ data: Default::default() }
    }
}

struct LazyList<T>{
    backing: Vec<T>,
    reached_end: bool,
    iterator: Box<dyn Iterator<Item = T>>,
}

impl<T> LazyList<T> {
    fn realize_one(&mut self) -> bool{
        if self.reached_end{
            false
        } else if let Some(value) = self.iterator.next() {
            self.backing.push(value);
            true
        } else{
            self.reached_end = true;
            false
        }
    }
    
    
    fn realize(&mut self, count: Option<usize>){
        match count {
            Some(count) => {
                for i in 0..count {
                    if ! self.realize_one() {
                        return;
                    }
                }
            },
            None => {
                while self.realize_one() {
                    
                }
            },
        }
    }

    fn fully_realized(&self) -> bool{
        self.reached_end
    }
}

impl <T> List<T> for RefCell<LazyList<T>>{
    fn len(&self) -> usize {
        let mut m = self.borrow_mut();
        if ! m.fully_realized(){
            m.realize(None);
        }
        m.backing.len()
    }
}
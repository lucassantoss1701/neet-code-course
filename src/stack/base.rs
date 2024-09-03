pub struct Stack{
    pub values: Vec<i64>,
    len: usize,
    next_value_index: usize,
}

 impl Stack{
    pub fn new(len: usize) -> Stack{
        Stack{
            values: vec![0; len],
            len,
            next_value_index: 0,
        }
    }
     pub fn pop(&mut self) {
         if self.next_value_index == 0 {
             return;
         }
         self.values[self.next_value_index - 1] = 0;
         self.next_value_index -= 1;
     }

     pub fn push(&mut self, value: i64) {
         if self.next_value_index > self.len {
             return;
         }
         self.values[self.next_value_index] = value;
         self.next_value_index += 1;
     }
}
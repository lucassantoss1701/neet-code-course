pub struct Stack{
    pub values: Vec<i64>,
    len: usize,
    last_value_index: usize,
}

 impl Stack{
    pub fn new(len: usize) -> Stack{
        Stack{
            values: vec![0; len],
            len,
            last_value_index: 0,
        }
    }
     pub fn pop(&mut self) {
         if self.last_value_index == 0 {
             return;
         }
         self.values[self.last_value_index - 1] = 0;
         self.last_value_index -= 1;
     }

     pub fn push(&mut self, value: i64) {
         if self.last_value_index >= self.len {
             return;
         }
         self.values[self.last_value_index] = value;
         self.last_value_index += 1;
     }
}
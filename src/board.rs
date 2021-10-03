pub struct board(Vec<Vec<u8>>);

impl board {
    pub fn init(n: usize) -> board {
        let mut v = Vec::with_capacity(n); 
        for i in 0..n {
            v[i] = Vec::with_capacity(n)
        }
        board(v)
    }
}
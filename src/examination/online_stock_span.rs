#[allow(dead_code)]
struct StockSpanner {
    data: Vec<i32>
}

impl StockSpanner {
    #[allow(dead_code)]
    fn new() -> Self {
        StockSpanner {
            data: vec![]
        }
    }
    
    #[allow(dead_code)]
    fn next(&mut self, price: i32) -> i32 {
        self.data.push(price);
        let mut count = 0;
        for i in self.data.iter().rev() {
            if i <= &price {
                count += 1;
            } else {
                return count;
            }
        }
        count
    }
}
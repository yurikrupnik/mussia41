trait Strategy {
    fn execute(&self, a: i32, b: i32) -> i32;
}

struct AddStrategy;

impl Strategy for AddStrategy {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

struct MultiplyStrategy;

impl Strategy for MultiplyStrategy {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a * b
    }
}

struct Context {
    strategy: Box<dyn Strategy>,
}

impl Context {
    fn new(strategy: Box<dyn Strategy>) -> Self {
        Context { strategy }
    }

    fn execute_strategy(&self, a: i32, b: i32) -> i32 {
        self.strategy.execute(a, b)
    }
}

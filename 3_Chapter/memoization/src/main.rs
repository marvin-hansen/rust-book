use std::cell::Cell;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Memo<T:, F: Fn() -> T> where T: Debug + Copy {
    cell: Cell<Option<T>>,
    func: F,
}

impl<T, F: Fn() -> T> Memo<T, F> where T: Debug + Copy {

    pub fn new(func: F) -> Memo<T, F> {
        Memo {
            cell: Cell::new(None),
            func,
        }
    }

    pub fn get(&self) -> T {
        return match self.cell.get() {
            Some(rc) => {
                rc
            }
            None => {
                let value = (self.func)(); // call the function
                self.cell.set(Some(value)); // move value into option & set it
                let rc = self.cell.get().unwrap(); // get & extract value
                rc
            }
        };
    }
}

fn main() {
    let a = 37846;
    let b = 19338;

    let memo = Memo::new(|| {
        println!("Computing...");
        a * b
    });

    // Computation is only executed once, at the first get() call
    println!("First call");
    println!("First result: {}", memo.get());

    println!("");
    println!("Second call");
    println!("Cached result: {}", memo.get());
}

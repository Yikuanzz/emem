use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // 多个所有者共享，并且可以修改数据
    let shared_data = Rc::new(RefCell::new(10));
    
    let a = Rc::clone(&shared_data);
    let b = Rc::clone(&shared_data);
    
    // 任意一个所有者都能修改内部值
    *a.borrow_mut() += 5;
    *b.borrow_mut() += 5;
    
    println!("{}", shared_data.borrow()); // 20
}
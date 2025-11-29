use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::io::{self, Write}; // Thêm thư viện io để nhập xuất

// Import logic từ các module con
mod insert;
mod delete;

// Định nghĩa màu sắc
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Red,
    Black,
}

// Định nghĩa Node
pub type NodePtr = Rc<RefCell<Node>>;
pub type WeakNodePtr = Weak<RefCell<Node>>;

pub struct Node {
    pub val: i32,
    pub color: Color,
    pub left: Option<NodePtr>,
    pub right: Option<NodePtr>,
    pub parent: Option<WeakNodePtr>,
}

impl Node {
    pub fn new(val: i32) -> NodePtr {
        Rc::new(RefCell::new(Node {
            val,
            color: Color::Red, // Node mới chèn luôn là Đỏ
            left: None,
            right: None,
            parent: None,
        }))
    }
}

// Cấu trúc cây
pub struct RedBlackTree {
    pub root: Option<NodePtr>,
}

impl RedBlackTree {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    // Hàm in cây theo thứ tự trước (Pre-order)
    pub fn print_preorder(&self) {
        if self.root.is_none() {
            println!("(Cây rỗng)");
            return;
        }
        Self::print_node(&self.root);
        println!();
    }

    fn print_node(node: &Option<NodePtr>) {
        if let Some(n) = node {
            let n_borrow = n.borrow();
            let color_char = if n_borrow.color == Color::Red { "R" } else { "B" };
            print!("{}({}) ", n_borrow.val, color_char);
            Self::print_node(&n_borrow.left);
            Self::print_node(&n_borrow.right);
        }
    }
    
    // Các hàm xoay cây (Dùng chung cho Insert và Delete)
    pub fn rotate_left(&mut self, x: NodePtr) {
        let y = x.borrow().right.clone().unwrap();
        
        x.borrow_mut().right = y.borrow().left.clone();
        if let Some(ref y_left) = y.borrow().left {
            y_left.borrow_mut().parent = Some(Rc::downgrade(&x));
        }

        y.borrow_mut().parent = x.borrow().parent.clone();
        if x.borrow().parent.is_none() {
            self.root = Some(y.clone());
        } else {
            let parent_weak = x.borrow().parent.clone().unwrap();
            let parent = parent_weak.upgrade().unwrap();
            let left_child_is_x = parent.borrow().left.as_ref().map_or(false, |l| Rc::ptr_eq(l, &x));
            
            if left_child_is_x {
                parent.borrow_mut().left = Some(y.clone());
            } else {
                parent.borrow_mut().right = Some(y.clone());
            }
        }

        y.borrow_mut().left = Some(x.clone());
        x.borrow_mut().parent = Some(Rc::downgrade(&y));
    }

    pub fn rotate_right(&mut self, x: NodePtr) {
        let y = x.borrow().left.clone().unwrap();

        x.borrow_mut().left = y.borrow().right.clone();
        if let Some(ref y_right) = y.borrow().right {
            y_right.borrow_mut().parent = Some(Rc::downgrade(&x));
        }

        y.borrow_mut().parent = x.borrow().parent.clone();
        if x.borrow().parent.is_none() {
            self.root = Some(y.clone());
        } else {
            let parent_weak = x.borrow().parent.clone().unwrap();
            let parent = parent_weak.upgrade().unwrap();
            let right_child_is_x = parent.borrow().right.as_ref().map_or(false, |r| Rc::ptr_eq(r, &x));

            if right_child_is_x {
                parent.borrow_mut().right = Some(y.clone());
            } else {
                parent.borrow_mut().left = Some(y.clone());
            }
        }

        y.borrow_mut().right = Some(x.clone());
        x.borrow_mut().parent = Some(Rc::downgrade(&y));
    }
}

// Hàm hỗ trợ đọc số từ bàn phím
fn read_int(prompt: &str) -> Option<i32> {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Đẩy buffer để in prompt ngay lập tức
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<i32>().ok()
}

fn main() {
    let mut tree = RedBlackTree::new();
    println!("=== CHƯƠNG TRÌNH CÂY ĐỎ ĐEN (RUST) ===");

    loop {
        println!("\n---------------- MENU ----------------");
        println!("1. Insert (Chèn số)");
        println!("2. Delete (Xóa số)");
        println!("0. Exit (Thoát)");
        print!("Chọn thao tác: ");
        io::stdout().flush().unwrap();

        let mut choice_str = String::new();
        io::stdin().read_line(&mut choice_str).expect("Lỗi đọc dòng");
        
        match choice_str.trim() {
            "1" => {
                if let Some(val) = read_int("Nhập số cần chèn: ") {
                    println!("-> Đang chèn {}...", val);
                    insert::insert(&mut tree, val);
                    print!("Cây hiện tại (Pre-order): ");
                    tree.print_preorder();
                } else {
                    println!("Lỗi: Vui lòng nhập một số nguyên hợp lệ.");
                }
            }
            "2" => {
                if let Some(val) = read_int("Nhập số cần xóa: ") {
                    println!("-> Đang xóa {}...", val);
                    delete::delete(&mut tree, val);
                    print!("Cây hiện tại (Pre-order): ");
                    tree.print_preorder();
                } else {
                    println!("Lỗi: Vui lòng nhập một số nguyên hợp lệ.");
                }
            }
            "0" => {
                println!("Tạm biệt!");
                break;
            }
            _ => {
                println!("Lựa chọn không hợp lệ. Vui lòng nhập 1, 2 hoặc 0.");
            }
        }
    }
}
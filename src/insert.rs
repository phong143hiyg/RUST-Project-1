use crate::{RedBlackTree, Node, Color, NodePtr};
use std::rc::Rc;

// Hàm insert chính [cite: 306-307]
pub fn insert(tree: &mut RedBlackTree, val: i32) {
    let z = Node::new(val);
    let mut y = None;
    let mut x = tree.root.clone();

    // Tìm vị trí chèn (giống BST thường)
    while let Some(node) = x {
        y = Some(node.clone());
        if val < node.borrow().val {
            x = node.borrow().left.clone();
        } else {
            x = node.borrow().right.clone();
        }
    }

    // Gắn node cha
    z.borrow_mut().parent = y.as_ref().map(|n| Rc::downgrade(n));

    if y.is_none() {
        tree.root = Some(z.clone());
    } else {
        let parent = y.unwrap();
        if val < parent.borrow().val {
            parent.borrow_mut().left = Some(z.clone());
        } else {
            parent.borrow_mut().right = Some(z.clone());
        }
    }

    // Khôi phục tính chất Đỏ-Đen
    insert_fixup(tree, z);
}

// Hàm khôi phục tính chất cây [cite: 308-323]
fn insert_fixup(tree: &mut RedBlackTree, mut k: NodePtr) {
    // Trong khi cha của k tồn tại và màu Đỏ
    loop {
        let parent_weak_opt = k.borrow().parent.clone();
        if parent_weak_opt.is_none() {
            break;
        }
        
        let parent = parent_weak_opt.unwrap().upgrade().unwrap();
        if parent.borrow().color == Color::Black {
            break;
        }

        let grandparent_weak = parent.borrow().parent.clone().unwrap();
        let grandparent = grandparent_weak.upgrade().unwrap();

        // Nếu cha là con trái của ông
        let is_parent_left = grandparent.borrow().left.as_ref().map_or(false, |l| Rc::ptr_eq(l, &parent));

        if is_parent_left {
            let uncle = grandparent.borrow().right.clone();
            // Case 1: Chú màu đỏ [cite: 157, 311]
            if uncle.as_ref().map_or(false, |u| u.borrow().color == Color::Red) {
                parent.borrow_mut().color = Color::Black;
                uncle.unwrap().borrow_mut().color = Color::Black;
                grandparent.borrow_mut().color = Color::Red;
                k = grandparent;
            } else {
                // Case 2: Chú màu đen, k là con phải (Triangle) -> Xoay trái tại cha [cite: 204, 317]
                let is_k_right_child = parent.borrow().right.as_ref().map_or(false, |r| Rc::ptr_eq(r, &k));
                if is_k_right_child {
                    k = parent.clone();
                    tree.rotate_left(k.clone());
                }
                // Case 3: Chú màu đen, k là con trái (Line) -> Đổi màu + Xoay phải tại ông [cite: 183, 320]
                let new_parent_weak = k.borrow().parent.clone().unwrap(); // Lấy lại cha (có thể đã đổi sau xoay)
                let new_parent = new_parent_weak.upgrade().unwrap();
                
                new_parent.borrow_mut().color = Color::Black;
                // Lấy lại ông (phải lấy từ parent mới vì cấu trúc đã thay đổi)
                let gp_weak_opt = new_parent.borrow().parent.clone();
                if let Some(gp_weak) = gp_weak_opt {
                     let gp = gp_weak.upgrade().unwrap();
                     gp.borrow_mut().color = Color::Red;
                     tree.rotate_right(gp);
                }
            }
        } else {
            // Đối xứng: Cha là con phải của ông
            let uncle = grandparent.borrow().left.clone();
            // Case 1: Chú đỏ
            if uncle.as_ref().map_or(false, |u| u.borrow().color == Color::Red) {
                parent.borrow_mut().color = Color::Black;
                uncle.unwrap().borrow_mut().color = Color::Black;
                grandparent.borrow_mut().color = Color::Red;
                k = grandparent;
            } else {
                // Case 2: Chú đen, k là con trái -> Xoay phải tại cha
                let is_k_left_child = parent.borrow().left.as_ref().map_or(false, |l| Rc::ptr_eq(l, &k));
                if is_k_left_child {
                    k = parent.clone();
                    tree.rotate_right(k.clone());
                }
                // Case 3: Chú đen, k là con phải -> Đổi màu + Xoay trái tại ông
                let new_parent_weak = k.borrow().parent.clone().unwrap();
                let new_parent = new_parent_weak.upgrade().unwrap();

                new_parent.borrow_mut().color = Color::Black;
                let gp_weak_opt = new_parent.borrow().parent.clone();
                if let Some(gp_weak) = gp_weak_opt {
                    let gp = gp_weak.upgrade().unwrap();
                    gp.borrow_mut().color = Color::Red;
                    tree.rotate_left(gp);
                }
            }
        }
    }
    // Gốc luôn là đen [cite: 29]
    if let Some(root) = &tree.root {
        root.borrow_mut().color = Color::Black;
    }
}
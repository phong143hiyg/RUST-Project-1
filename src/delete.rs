use crate::{RedBlackTree, Color, NodePtr};
use std::rc::Rc;

// --- HÀM SỬA LỖI ---
// Hàm trợ giúp để so sánh địa chỉ bộ nhớ của hai Node
// Trả về true nếu chúng KHÁC nhau về mặt địa chỉ (pointer)
fn is_different_node(a: &Option<NodePtr>, b: &Option<NodePtr>) -> bool {
    match (a, b) {
        (Some(a_ptr), Some(b_ptr)) => !Rc::ptr_eq(a_ptr, b_ptr), // So sánh con trỏ: Nếu khác địa chỉ thì return true
        (None, None) => false, // Cả hai đều là None (NIL) => Coi là giống nhau
        _ => true, // Một bên có, một bên không => Khác nhau
    }
}

// Hàm xóa public
pub fn delete(tree: &mut RedBlackTree, key: i32) {
    let z = find_node(tree, key);
    if z.is_none() { return; }
    let z = z.unwrap();

    let mut y = z.clone(); // Node thực sự bị xóa hoặc di chuyển
    let mut y_original_color = y.borrow().color;
    let x: Option<NodePtr>; // Node thay thế vị trí của y
    let x_parent: Option<NodePtr>; // Cha của x (cần thiết vì x có thể là None/NIL)

    if z.borrow().left.is_none() {
        x = z.borrow().right.clone();
        x_parent = z.borrow().parent.as_ref().map(|p| p.upgrade().unwrap());
        transplant(tree, z.clone(), z.borrow().right.clone());
    } else if z.borrow().right.is_none() {
        x = z.borrow().left.clone();
        x_parent = z.borrow().parent.as_ref().map(|p| p.upgrade().unwrap());
        transplant(tree, z.clone(), z.borrow().left.clone());
    } else {
        // Có 2 con, tìm successor
        let successor = minimum(z.borrow().right.clone().unwrap());
        y = successor.clone();
        y_original_color = y.borrow().color;
        x = y.borrow().right.clone();
        
        if y.borrow().parent.as_ref().unwrap().upgrade().unwrap().as_ptr() == z.as_ptr() {
             x_parent = Some(y.clone()); // Trường hợp đặc biệt khi y là con trực tiếp của z
        } else {
             x_parent = y.borrow().parent.as_ref().map(|p| p.upgrade().unwrap());
             transplant(tree, y.clone(), y.borrow().right.clone());
             y.borrow_mut().right = z.borrow().right.clone();
             if let Some(right) = &y.borrow().right {
                 right.borrow_mut().parent = Some(Rc::downgrade(&y));
             }
        }
        transplant(tree, z.clone(), Some(y.clone()));
        y.borrow_mut().left = z.borrow().left.clone();
        y.borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&y));
        y.borrow_mut().color = z.borrow().color;
    }

    // Nếu node bị xóa màu đen, cần khôi phục
    if y_original_color == Color::Black {
        delete_fixup(tree, x, x_parent);
    }
}

// Thay thế cây con u bằng cây con v
fn transplant(tree: &mut RedBlackTree, u: NodePtr, v: Option<NodePtr>) {
    if u.borrow().parent.is_none() {
        tree.root = v.clone();
    } else {
        let parent = u.borrow().parent.clone().unwrap().upgrade().unwrap();
        let is_left = parent.borrow().left.as_ref().map_or(false, |l| Rc::ptr_eq(l, &u));
        if is_left {
            parent.borrow_mut().left = v.clone();
        } else {
            parent.borrow_mut().right = v.clone();
        }
    }
    if let Some(ref v_node) = v {
        v_node.borrow_mut().parent = u.borrow().parent.clone();
    }
}

fn find_node(tree: &RedBlackTree, key: i32) -> Option<NodePtr> {
    let mut curr = tree.root.clone();
    while let Some(node) = curr {
        let val = node.borrow().val;
        if key == val { return Some(node); }
        else if key < val { curr = node.borrow().left.clone(); }
        else { curr = node.borrow().right.clone(); }
    }
    None
}

fn minimum(node: NodePtr) -> NodePtr {
    let mut curr = node;
    while let Some(left) = curr.clone().borrow().left.clone() {
        curr = left;
    }
    curr
}

fn delete_fixup(tree: &mut RedBlackTree, mut x: Option<NodePtr>, mut x_parent: Option<NodePtr>) {
    // --- SỬA LỖI Ở ĐÂY: Sử dụng hàm is_different_node thay vì != ---
    while is_different_node(&x, &tree.root) && x.as_ref().map_or(true, |n| n.borrow().color == Color::Black) {
        if x_parent.is_none() { break; } // An toàn
        let parent = x_parent.clone().unwrap();
        
        let is_x_left = match &x {
            Some(node) => parent.borrow().left.as_ref().map_or(false, |l| Rc::ptr_eq(l, node)),
            None => parent.borrow().left.is_none(),
        };

        if is_x_left {
            let mut w = parent.borrow().right.clone(); // Anh em của x
            
            // Case 1: Anh em màu đỏ
            if w.as_ref().map_or(false, |n| n.borrow().color == Color::Red) {
                w.as_ref().unwrap().borrow_mut().color = Color::Black;
                parent.borrow_mut().color = Color::Red;
                tree.rotate_left(parent.clone());
                w = parent.borrow().right.clone(); // Cập nhật lại w
            }

            // Kiểm tra màu của các con w
            let w_unwrap = w.clone().unwrap(); // w chắc chắn không None do tính chất RB
            let w_left_black = w_unwrap.borrow().left.as_ref().map_or(true, |n| n.borrow().color == Color::Black);
            let w_right_black = w_unwrap.borrow().right.as_ref().map_or(true, |n| n.borrow().color == Color::Black);

            if w_left_black && w_right_black {
                // Case 2: Anh em đen và 2 con đều đen
                w_unwrap.borrow_mut().color = Color::Red;
                x = Some(parent.clone());
                x_parent = parent.borrow().parent.as_ref().map(|p| p.upgrade().unwrap());
            } else {
                let mut w_curr = w_unwrap;
                if w_right_black {
                    // Case 3: Anh em đen, con trái đỏ, con phải đen
                    if let Some(left) = w_curr.borrow().left.clone() {
                        left.borrow_mut().color = Color::Black;
                    }
                    w_curr.borrow_mut().color = Color::Red;
                    tree.rotate_right(w_curr.clone());
                    w_curr = parent.borrow().right.clone().unwrap();
                }
                // Case 4: Anh em đen, con phải đỏ
                w_curr.borrow_mut().color = parent.borrow().color;
                parent.borrow_mut().color = Color::Black;
                if let Some(right) = w_curr.borrow().right.clone() {
                    right.borrow_mut().color = Color::Black;
                }
                tree.rotate_left(parent.clone());
                x = tree.root.clone(); // Thoát vòng lặp
                x_parent = None;
            }
        } else {
            // Đối xứng: x là con phải
            let mut w = parent.borrow().left.clone();

            // Case 1 (Symmetric)
            if w.as_ref().map_or(false, |n| n.borrow().color == Color::Red) {
                w.as_ref().unwrap().borrow_mut().color = Color::Black;
                parent.borrow_mut().color = Color::Red;
                tree.rotate_right(parent.clone());
                w = parent.borrow().left.clone();
            }

            let w_unwrap = w.clone().unwrap();
            let w_left_black = w_unwrap.borrow().left.as_ref().map_or(true, |n| n.borrow().color == Color::Black);
            let w_right_black = w_unwrap.borrow().right.as_ref().map_or(true, |n| n.borrow().color == Color::Black);

            if w_left_black && w_right_black {
                // Case 2 (Symmetric)
                w_unwrap.borrow_mut().color = Color::Red;
                x = Some(parent.clone());
                x_parent = parent.borrow().parent.as_ref().map(|p| p.upgrade().unwrap());
            } else {
                let mut w_curr = w_unwrap;
                if w_left_black {
                    // Case 3 (Symmetric)
                    if let Some(right) = w_curr.borrow().right.clone() {
                        right.borrow_mut().color = Color::Black;
                    }
                    w_curr.borrow_mut().color = Color::Red;
                    tree.rotate_left(w_curr.clone());
                    w_curr = parent.borrow().left.clone().unwrap();
                }
                // Case 4 (Symmetric)
                w_curr.borrow_mut().color = parent.borrow().color;
                parent.borrow_mut().color = Color::Black;
                if let Some(left) = w_curr.borrow().left.clone() {
                    left.borrow_mut().color = Color::Black;
                }
                tree.rotate_right(parent.clone());
                x = tree.root.clone();
                x_parent = None;
            }
        }
    }
    if let Some(node) = x {
        node.borrow_mut().color = Color::Black;
    }
}
use crate::LinkedList;

/// # linked_list
/// 連結リスト作成用マクロ
macro_rules! linked_list {
    ( $val:expr, $( $vals:expr ), * $(,)* ) => {{
        LinkedList::Node {
            val: $val,
            next: Box::new( linked_list!( $( $vals, )* ) ),
        }
    }};
    ( $val:expr $(,)* ) => {{
        LinkedList::Node {
            val: $val,
            next: Box::new( linked_list!() ),
        }
    }};
    () => {{
        LinkedList::Null   
    }};
}

#[test]
fn test_access() {
    let mut list = linked_list!(1, 2, 3, 4);
    println!("{:?}", list);
    
    let third_elem = list.nth(4).unwrap();
    println!("{:?}", third_elem);
    
    third_elem.remove();
    println!("{:?}", third_elem);
    
    println!("{:?}", list);
}

#[test]
fn test_replace() {
    let mut list = linked_list!(1, 2, 3, 4);

    if let LinkedList::Node{ val, next } = &mut list {
        println!("置き換え前");
        println!("val: {}", val);
        println!("next: {:?}", next);
    
        // 置き換える
        let new_node = linked_list!(100);
        next.replace(new_node);
    
        println!("置き換え後");
        println!("val: {}", val);
        println!("next: {:?}", next);
    }
    
    println!("{:?}", list);
    
    if let LinkedList::Node{ val:_, next } = &mut list {
        next.remove();
    }
    
    println!("{:?}", list);
    
    list.remove();
    
    println!("{:?}", list);
}

#[test]
fn test_search() {
    let list = linked_list!(1, 6, 4, 7, 8, 3, 10, 2);
    eprintln!("list: {:?}", list);
    
    let find_3 = list.find(3);
    eprintln!("list.find(3) = {:?}", find_3);
    
    let find_2 = list.find(2);
    eprintln!("list.find(2) = {:?}", find_2);
    
    let find_100 = list.find(100);
    eprintln!("list.find(100) = {:?}", find_100);
}

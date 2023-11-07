use double_linked_list::double_linked_list::*;

fn main() {
    let mut list = DoubleLinkedList::<usize>::new();

    for i in 0..10 {
        list.insert_head(i);
    }

    println!("{:?}", &list);

    // 5番目の要素の後に挿入
    {
        let ptr = list.nth(4).unwrap();

        unsafe { insert_next(ptr, 200) };
        unsafe { insert_prev(ptr, 100) };

        println!("{:?}", &list);
    }

    // 5番目の要素を削除
    {
        let ptr = list.nth(5).unwrap();

        list.delete(ptr);

        println!("{:?}", &list);
    }

    // 逆向きにトラバース
    {
        let mut tail = list.tail;

        while let Some(tail_ptr) = tail {
            print!("{:?} -> ", unsafe { &*tail_ptr });
            unsafe {
                tail = (*tail_ptr).prev;
            }
        }

        println!("End");
    }
}

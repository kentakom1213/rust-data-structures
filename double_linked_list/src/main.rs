use double_linked_list::double_linked_list::*;

fn main() {
    let mut list = DoubleLinkedList::<usize>::new();

    for i in 0..10 {
        list.insert_head(i);
    }

    println!("{:?}", &list);

    // 連番で取得
    {
        let mut ptr = list.head;

        while let Some(rptr) = ptr {
            println!("{:?}", ptr);
            ptr = next(rptr);
        }
    }

    // 5番目の要素の後に挿入
    {
        let ptr = list.nth(4).unwrap();

        insert_next(ptr, 200);
        insert_prev(ptr, 100);

        println!("{:?}", &list);
    }

    // 5番目の要素を削除
    {
        let ptr = list.nth(5).unwrap();

        // delete(ptr);

        println!("{:?}", &list);
    }
}

use double_linked_list::double_linked_list::*;

const N: usize = 10;

#[test]
fn test_insert_head() {
    let mut dll = DoubleLinkedList::new();

    for i in 0..N {
        dll.insert_head(i);
    }

    let actual = format!("{:?}", &dll);
    let expect = "LinkedList([Node(9), Node(8), Node(7), Node(6), Node(5), Node(4), Node(3), Node(2), Node(1), Node(0), ])";

    assert_eq!(&actual[..], expect);
}

#[test]
fn test_insert_tail() {
    let mut dll = DoubleLinkedList::new();

    for i in 0..N {
        dll.insert_tail(i);
    }

    let actual = format!("{:?}", &dll);
    let expect = "LinkedList([Node(0), Node(1), Node(2), Node(3), Node(4), Node(5), Node(6), Node(7), Node(8), Node(9), ])";

    assert_eq!(&actual[..], expect);
}

#[test]
fn test_delete_head() {
    let mut dll = DoubleLinkedList::new();

    for i in 0..N {
        dll.insert_tail(i);
    }

    println!("削除前");
    println!("{:?}", &dll);

    assert_eq!(dll.delete_head(), Some(0));
    assert_eq!(dll.delete_head(), Some(1));
    assert_eq!(dll.delete_head(), Some(2));

    println!("先頭3つを削除");
    println!("{:?}", &dll);

    assert_eq!(dll.delete_tail(), Some(9));
    assert_eq!(dll.delete_tail(), Some(8));
    assert_eq!(dll.delete_tail(), Some(7));

    println!("末尾3つを削除");
    println!("{:?}", &dll);

    assert_eq!(dll.delete_head(), Some(3));
    assert_eq!(dll.delete_head(), Some(4));
    assert_eq!(dll.delete_head(), Some(5));
    assert_eq!(dll.delete_head(), Some(6));

    println!("すべての要素を削除");
    println!("{:?}", &dll);

    assert_eq!(dll.delete_tail(), None);
}

#[test]
fn test_find() {
    let mut dll = DoubleLinkedList::<&str>::new();

    dll.insert_tail("alpha");
    dll.insert_tail("beta");
    dll.insert_tail("gamma");
    dll.insert_tail("delta");
    dll.insert_tail("epsilon");

    println!("{:?}", &dll);
    assert_eq!(dll.len(), 5);

    {
        // betaを検索
        let beta = dll.find(&"beta");

        // 削除
        if let Some(ptr) = beta {
            dll.delete(ptr);
        }
    }

    println!("{:?}", &dll);
    assert_eq!(dll.len(), 4);

    {
        // epsilonを検索
        let eps = dll.find(&"epsilon");

        // 削除
        if let Some(ptr) = eps {
            dll.delete(ptr);
        }
    }

    println!("{:?}", &dll);
    assert_eq!(dll.len(), 3);

    {
        // alphaを検索
        let alpha = dll.find(&"alpha");

        // 削除
        if let Some(ptr) = alpha {
            dll.delete(ptr);
        }
    }

    println!("{:?}", &dll);
    assert_eq!(dll.len(), 2);
}

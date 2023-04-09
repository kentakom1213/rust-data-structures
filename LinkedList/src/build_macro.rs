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

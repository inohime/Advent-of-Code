mod utils {
    #[macro_export]
    macro_rules! set {
        ($ ($key: expr), *) => {{
            let set = std::collections::HashSet::new();
            $(set.insert($key);)*
            set
        }}
    }
    #[macro_export]
    macro_rules! deque {
        ($ ($key: expr), *) => {{
            let queue = std::collections::VecDeque::new();
            $(queue.push($key);)*
            queue
        }}
    }
    #[macro_export]
    macro_rules! map {
        ($ ($key: expr, $val: expr ), *) => {{
            let mut map = ::std::collections::HashMap::new();
            $(map.insert($key, $val);)*
            map
        }}
    }
}

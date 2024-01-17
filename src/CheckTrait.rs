#[cfg(test)]
mod check_trait {
    use std::cell::{OnceCell, RefCell};
    use std::sync::OnceLock;

    struct Op {
        text: String,
    }
    fn is_sync<T: Sync>() {}
    #[test]
    fn check_trait() {
        is_sync::<String>();
        is_sync::<Vec<i32>>();
        is_sync::<OnceLock<Vec<i32>>>();
        // is_sync::<OnceCell<Op>>(); //error[E0277]: `std::cell::OnceCell<check_trait::Op>` cannot be shared between threads safely
        // is_sync::<OnceCell<String>>();
        // is_sync::<OnceCell<Vec<i32>>>();
    }
}

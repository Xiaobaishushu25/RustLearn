use std::sync::OnceLock;
use tokio::task::block_in_place;

static DB: OnceLock<Option<i32>> = OnceLock::new();
fn create_db() {
    DB.get_or_init(|| {
        block_in_place(|| async { Some(1) });
        Some(0)
    });
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_log() {
        let x: Result<i32, &'static str> = Err("test");
        x.map_err(|e| println!("出现错误{}", e.to_string()))
            .expect("控制台报错");
    }
}

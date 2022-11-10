#[test]
fn ready() {
    println!("it works!")
}

#[tokio::test]
async fn test_files() {
    let path = PathBuf::from("sqlite");
    let file_db = DiskMap::new(&path).unwrap();
    let key = "key";
    let value = "value".to_string();
    file_db.insert(key, value);
    println!("{:?}", file_db.get("key"));

    // file_db.test().await.unwrap()
}

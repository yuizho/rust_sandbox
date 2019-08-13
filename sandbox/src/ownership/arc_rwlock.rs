use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, RwLock};

// 途中で?を使うので、
fn main() -> Result<(), Box<dyn Error>> {
    let dogs: HashSet<_> = ["柴", "トイプードル"].iter().cloned().collect();
    // HashSetを可変にするためにRwLockで包み、スレッド間共有のためにArcで包む
    let dogs = Arc::new(RwLock::new(dogs)); // Arc<RwLock<HashSet<& 'static str>>>
    fn stringify(x: impl ToString) -> String {
        x.to_string()
    }
    {
        let ds = dogs.read().map_err(stringify)?; // readロックを取得
        assert!(ds.contains("柴"));
        assert!(ds.contains("トイプードル"))
    } // dsがスコープを外れロックが解除される
    dogs.write().map_err(stringify)?.insert("ブル・テリア");

    let dogs1 = Arc::clone(&dogs);
    std::thread::spawn(move ||
        // Arc::cloneで複製したArcポインタを渡す子でHashSetを共有
        // writeロックを取得して別スレッドから要素を追加
        dogs1.write().map(|mut ds| ds.insert("コーギー")).map_err(stringify))
    .join()
    .expect("Thread error")?;

    // このスレッドと別スレッドの両方で追加した要素が見える
    println!("{:?}", dogs.read().map_err(stringify)?);

    Ok(())
}

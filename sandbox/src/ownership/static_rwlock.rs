use lazy_static::lazy_static;
use std::collections::HashSet;
use std::error::Error;
use std::sync::RwLock;

// lazy_staticを使うとstatic変数に関数呼び出しを含む初期化コードをもたせられる
lazy_static! {
    pub static ref DOGS: RwLock<HashSet<&'static str>> = {
        let dogs = ["柴", "トイプードル"].iter().cloned().collect();
        RwLock::new(dogs)
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    {
        let dogs = DOGS.read()?;
        assert!(dogs.contains("柴"));
        assert!(dogs.contains("トイプードル"));
    } // dogsがスコープ外になり、readロックが解除される

    fn stringify(x: impl ToString) -> String {
        x.to_string()
    }

    DOGS.write()?.insert("ブル・テリア");

    std::thread::spawn(move ||
        // writeロックを取得して別スレッドから要素を追加
        DOGS.write().map(|mut ds| ds.insert("コーギー")).map_err(stringify))
    .join()
    .expect("Thread error")?;

    // このスレッドと別スレッドの両方で追加した要素が見える
    println!("{:?}", DOGS.read()?);

    Ok(())
}

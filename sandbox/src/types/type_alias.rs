fn main() {
    type UserName = String;
    type Id = i64;
    type Timestamp = i64;
    type User = (Id, UserName, Timestamp);

    fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
        (id, name, created)
    }

    let id = 400;
    let now = 4567890123;
    // これでも行けるけど、順番が変わるとエラーになってしまう
    let user = new_user(String::from("mika"), id, now);
    println!("{:?}", user);

    // タプル構造体を使った改善
    #[derive(Debug)]
    struct UserName2(String);
    #[derive(Debug)]
    struct Id2(i64);
    #[derive(Debug)]
    struct Timestamp2(i64);
    type User2 = (Id2, UserName2, Timestamp2);

    fn new_user2(name: UserName2, id: Id2, created: Timestamp2) -> User2 {
        (id, name, created)
    }

    let id = Id2(400);
    let now = Timestamp2(4567890123);
    // idとnowを入れ替えるとコンパイルエラーとなる
    let user = new_user2(UserName2(String::from("kazuki")), id, now);
    println!("{:?}", user);
}

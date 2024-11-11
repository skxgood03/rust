use std::collections::HashMap;
fn main(){
    //创建普通的哈希==========================
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    //
    // scores.insert(String::from("Yellow"), 50);

    // 使用vector构建哈希
    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    // let scores:HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 访问哈希 map 中的值 访问哈希 map 中储存的蓝队分数+++++++++++++++++++++++++++++++++++++
    // let mut scores = HashMap::new();
    //
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);
    // println!("{:?}", score); // Some(10)
    //
    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }
    // Yellow: 50
    // Blue: 10

    // 更新=====================================================================
    // let mut scores = HashMap::new();
    //
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);
    // println!("{:?}", scores) //{"Blue": 25}

    // 判断键是否有值 entry++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    // 检查黄队的键是否关联了一个值。如果没有，就插入值 50，对于蓝队也是如此
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10); //新增
    // scores.entry(String::from("Yellow")).or_insert(50); //如果键不存在就插入
    // scores.entry(String::from("Blue")).or_insert(50);
    //
    // println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    //，split_whitespace 方法用于将字符串按空白字符分割成多个子字符串。空白字符包括空格、制表符、换行符等。
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); //如果没有键，就插入一个值为 0 的条目，并返回对这个新条目的引用。
        *count += 1;
    }

    println!("{:?}", map); //{"world": 2, "hello": 1, "wonderful": 1}
}
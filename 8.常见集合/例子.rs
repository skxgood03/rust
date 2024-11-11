use std::collections::HashMap;
//求平均数 众数 中位数
fn main() {
    let mut   v = vec![1,2,4,5,6];
    let lens = v.len();
    let mut num_size = 0;
    for i in v.iter(){
        num_size += i;
    }
    println!("平均数{}",num_size/lens);

    v.sort(); //排序
    let median = if lens%2 ==1{
        //如果是奇数，则中位数为中间那个数
        v[lens/2]
    }else{
        //如果是偶数，则中位数为中间两个数的平均值
        (v[lens/2]+v[lens/2-1])/2

    };
    print!("中位数{}",median);
    let mut   v = vec![1,2,4,5,6,5];
    let mut scores = HashMap::new();
    for element in &v{
        *scores.entry(element).or_insert(0) += 1;
    }
    // 打印 HashMap 以验证结果
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // 找到出现次数最多的元素
    let mut max_count = 0;
    let mut mode = None;

    for (key, count) in &scores {
        if *count > max_count {
            max_count = *count;
            mode = Some(*key);
        }
    }
    match mode {
        Some(key) => println!("The mode is {}", key),
        None => println!("No mode found"),
    }


}

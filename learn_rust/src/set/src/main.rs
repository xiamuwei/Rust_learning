use std::collections::HashMap;
fn main() {
    // println!("Hello, world!");
    hashMap_test();
}


// 常见集合

// vector :只能存放相同类型的值
fn vector_test(){
    // 使用Vec::new函数创建Vector, Vector是泛型实现的
    let mut v :Vec<i32> = Vec::new();
    // 使用vec! 宏来创建vector
    let mut v2 = vec![1,2,3,4,5];
    // 增加
    v.push(2);
    v.push(3);
    v.push(4);

    v.append(&mut v2);

    // 删除
    v.pop();

    // v.clear();

    // 根据下标获取值
    // println!("v = {}",v[0]);

    // 遍历vector
    for i in v{
        println!("i = {}",i)
    }
}

// 字符串
fn string_test(){
    let str1 = String::from("this is String");
    let str2 = "test".to_string();
    println!("str1 = {}, str2 = {}",str1 ,str2);

    // 拼接字符串
    // 使用+
    // let str3 = str1+ &str2; 此处发生强制类型转化,将&String 转换成&str
    // println!("{}",str3);

    // 使用format,同时拼接多个字符串
    let str3 = format!("{} + {}", str1, str2);
    println!("{}",str3);
}


// hashMap
fn hashMap_test(){
    let mut map: HashMap<i32, i32> = HashMap::new();
    // 增加
    map.insert(1, 100);
    map.insert(2, 101);
    map.insert(3, 102);

    // 删

    // 改变
        // 覆盖
        map.insert(1, 200);
        // 只有键没有对应值时插入
        map.entry(5).or_insert(500);
        // 根据旧键更新值
        let x = map.entry(1).or_insert(100);
        *x += 1;
    // 遍历
    for (i,k) in map{
        println!("map[{}] = {}",i,k);
    }
}
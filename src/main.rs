use std::io;
use rand::Rng;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::thread;
use std::time::Duration;

fn main(){
    loop {
        let apps: BTreeMap<i32, &str> = [(1,"水仙花"),(2,"字符串切片"),(3,"元音字母计数"),(4,"猜数字"),(5,"句子单词统计"),(6,"倒计时程序"),(7,"购物车"),(8,"退出")].iter().cloned().collect();
        print!("你要运行哪个程序：");
        for (key, value) in &apps {
            print!("{}: {} ", key, value);
        }
        println!();
        println!("请输入对应的数字：");

        let mut choice: String = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        
        let choice: usize = match choice.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("输入错误，请输入数字！\n");
                continue;
            },
        };
        
        // 检查输入的数字是否在范围内
        match choice {
            1 => println!("你选择了{}\n",apps.get(&1).unwrap()),
            2 => println!("你选择了{}\n",apps.get(&2).unwrap()),
            3 => println!("你选择了{}\n",apps.get(&3).unwrap()),
            4 => println!("你选择了{}\n",apps.get(&4).unwrap()),
            5 => println!("你选择了{}\n",apps.get(&5).unwrap()),
            6 => println!("你选择了{}\n",apps.get(&6).unwrap()),
            7 => println!("你选择了{}\n",apps.get(&7).unwrap()),
            8 => {
                println!("退出程序");
                break;
            },
            _ => {
                println!("输入错误，请输入1-6之间的数字！\n");
                continue;
            },
        }
        
        println!("你选择的程序是：{}\n", choice);
        
        // 运行对应的程序
        match choice {
            1 => daffodil(),

            2 => 字符串切片(),

            3 => {
                let mut input: String = String::new();
                print!("请输入一个字符串：");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line\n");
                let count = count_vowels(&input);
                println!("元音字母的个数为：{}\n", count);
            },

            4 => guess_number(),

            5 => {
                println!("请输入一段文本：");

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("读取失败");
            
                let word_counts = count_words(&input);
                let mut words_count = 0;
                println!("\n单词出现次数：");
                for (word, count) in word_counts {
                    println!("{}: {}", word, count);
                    words_count += count;
                }
                println!("总单词数：{}\n", words_count);
            },

            6 => {
                println!("请输入倒计时的秒数：");

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("读取失败");
            
                let seconds: u32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("请输入一个有效的正整数！");
                        return;
                    }
                };
            
                println!("倒计时开始：");
                countdown(seconds);
            },

            7 => {
                let products: Vec<(&'static str, f64)> = vec![
                    ("苹果", 3.0),
                    ("香蕉", 2.5),
                    ("橙子", 4.0),
                    ("葡萄", 5.0),
                ];

                let mut cart: HashMap<String, (f64, usize)> = HashMap::new();

                loop {
                    println!("\n请选择操作：");
                    println!("1. 查看商品列表");
                    println!("2. 添加商品到购物车");
                    println!("3. 查看购物车");
                    println!("4. 清空购物车");
                    println!("5. 退出");

                    let mut choice: String = String::new();
                    io::stdin()
                        .read_line(&mut choice)
                        .expect("读取失败");
                    let choice: &str = choice.trim();

                    match choice {
                        "1" => cart::view_products(&products),
                        "2" => cart::add_to_cart(&products, &mut cart),
                        "3" => cart::view_cart(&cart),
                        "4" => cart::clear_cart(&mut cart),
                        "5" => {
                            println!("退出程序");
                            break;
                        }
                        _ => println!("无效的选择，请重新输入！"),
                    }
                }
            }

            _ => (),
        }   
    }   
}

fn daffodil(){
    print!("水仙花数：");
    for i in 100..1000{
        let a: i32 = i32::pow(i/100,3);
        let b: i32 = i32::pow((i / 10) % 10,3);
        let c: i32 = i32::pow(i%10,3);
        
        if a + b + c == i{
            print!("{} ", i);
        }
    }
    println!();
}

fn 字符串切片(){
    let s: &'static str = "hello, world\n";
    let hello = &s[0..=4];
    let world = &s[6..=11];
    println!("{} {}", hello, world);

}

fn count_vowels(input: &str) -> usize {
    // 统计元音字母的个数
    let mut count: usize = 0;
    for i in input.chars(){
        if "aeiouAEIOU".contains(i){
            count += 1;
        }
    }
    count

}

fn guess_number(){
    println!("猜数字游戏");
    // 生成一个随机数
    let secret_number: usize = rand::rng().random_range(1..=100);
    
    loop {
        println!("请输入你的猜数：");
        // 读取用户输入
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("请输入数字！\n");

        let guess: usize = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        // 输出用户输入的数字
        println!("你猜的数字是：{}", guess);
        // 判断用户输入的数字和随机数的大小关系
        // 使用模式匹配来判断
        match (guess, secret_number) {
            (a,b) if a == b => {
                println!("猜对了！\n");
                break;
            },
            (a,b) if a < b => println!("猜小了！\n"),
            (a,b) if a > b => println!("猜大了！\n"),
            _ => println!("出错了！\n"),
        }   
    }
}

fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();

    // 在这里实现单词计数逻辑
    for word in text.split_whitespace(){
        let word = word.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;    
    }
    counts
}

fn countdown(seconds: u32) {
    for i in (1..=seconds).rev() {
        println!("{}", i);
        thread::sleep(Duration::from_secs(1)); // 暂停一秒
    }
    println!("倒计时结束！");
}

// 购物车模块
mod cart {
    use std::collections::HashMap;
    pub fn view_products(products: &Vec<(&str, f64)>) {
        println!("\n商品列表：");
        for (i, (name, price)) in products.iter().enumerate() {
            println!("{}. {} - {:.2} 元", i + 1, name, price);
        }
    }

    pub fn add_to_cart(products: &Vec<(&str, f64)>, cart: &mut HashMap<String, (f64, usize)>) {
        // 在这里实现添加商品到购物车的逻辑
        println!("\n请输入要添加的商品编号：");
        let mut input: String = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("读取失败");
        let index: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个有效的商品编号！");
                return;
            }
        };
        if index == 0 || index > products.len() {
            println!("商品编号无效！");
            return;
        }
        let (name, price) = products[index - 1];
        let entry = cart.entry(name.to_string()).or_insert((0.0, 0));
        entry.0 = price;
        entry.1 += 1;
        println!("已将 {} 添加到购物车！", name);
        
    }

    pub fn view_cart(cart: &HashMap<String, (f64, usize)>) {
        println!("\n购物车内容：");
        if cart.is_empty() {
            println!("购物车为空！");
            return;
        }
        for (name, (price, quantity)) in cart.iter() {
            println!("{} - {:.2} 元 x {} = {:.2} 元", name, price, quantity, price * (*quantity as f64));
        }
        let total: f64 = cart.values().map(|(price, quantity)| price * (*quantity as f64)).sum();
        println!("总计：{:.2} 元", total);
        // 在这里实现查看购物车的逻辑
    }

    pub fn clear_cart(cart: &mut HashMap<String, (f64, usize)>) {
        println!("\n清空购物车！");
        cart.clear();
    }
}
use std::fmt::Display;
//就像泛型类型参数，泛型生命周期参数需要声明在函数名和参数列表间的尖括号中。
// 生命周期注解告诉 Rust 多个引用的泛型生命周期参数如何相互联系的。

// 这里我们想要告诉 Rust longest 函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn lifetime_complex_test() {
    println!(
        "{}",
        "------------lifetime_complex_test start-------------------"
    );
    // 函数签名中的生命周期注解
    lifetime_grammar_one();

    lifetime_grammar_two();

    //结构体定义中的生命周期注解
    lifetime_grammar_three();

    //生命周期省略
    lifetime_grammar_four();

    lifetime_grammar_five();
}

fn lifetime_grammar_one() {
    println!(
        "{}",
        "------------lifetime_grammar_one start-------------------"
    );
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    /* This will cause:  borrowed value does not live long enough
     let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        //longest return a refer, but it does not live long enough
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    */
}

fn lifetime_grammar_two() {
    println!(
        "{}",
        "------------lifetime_grammar_two start-------------------"
    );
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let s1;
    {
        s1 = string1.as_str();
        let s2 = string2.as_str();
        //The lifetime of s1,s2 not equal
        let result = longest(s1, s2);
        println!("The longest string is {}", result);
    }
    println!("s1 string is {}", s1);
    // It will cause error
    // println!("s2 string is {}",s2);
}

//定义包含引用的结构体，需要为结构体定义中的每一个引用添加生命周期注解
//这个注解意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久。
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetime_grammar_three() {
    println!(
        "{}",
        "------------lifetime_grammar_three start-------------------"
    );
    let novel = String::from("Call me Ishmael. Some years ago...");
    let novelpart = novel.as_str();
    let _i = ImportantExcerpt { part: novelpart };
    println!("{}",_i.part)
}

fn lifetime_grammar_four() {
    println!(
        "{}",
        "------------lifetime_grammar_two start-------------------"
    );
    /*
    第一条规则是每一个是引用的参数都有它自己的生命周期参数。
    换句话说就是，有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，
    有两个引用参数的函数有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。

    第二条规则是如果只有一个输入生命周期参数，
    那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。

    */
}

fn lifetime_grammar_five() {
    println!(
        "{}",
        "------------lifetime_grammar_five start-------------------"
    );
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let string3 = longest_with_an_announcement(string1.as_str(), string2.as_str(), 50);

    println!("string3 value is {}", string3);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

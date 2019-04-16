struct Student<'a> {
    name: &'a str,
    active: bool,
}

/*
第三条规则是如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为 &self 或 &mut self，
那么 self 的生命周期被赋给所有输出生命周期参数。这使得方法更容易读写，因为只需更少的符号。
下面这个例子不适用
*/
impl<'a> Student<'a> {
    fn test<'b>(&'b self, str1: &'b str) -> &'b str {
        if self.active {
            self.name
        } else {
            str1
        }
    }
}

pub fn lifetime_self_demo() {
    println!(
        "{}",
        "----------lifetime_self_demo end  --------------------"
    );
    let string1 = String::from("lifetime");
    let string2 = String::from("demo");
    let name = string1.as_str();
    let str1 = string2.as_str();
    let student = Student {
        name,
        active: false,
    };

    let str2;
    {
        str2 = student.test(str1);
    }
    println!("{}", str2);

    /*
    This will cause error: temporary value dropped here while still borrowed
    temporary value does not live long enough
    `String::from("wenbin")` does not live long enough
    let str3 = String::from("wenbin").as_str();
    println!("{}",str3);
    */
}

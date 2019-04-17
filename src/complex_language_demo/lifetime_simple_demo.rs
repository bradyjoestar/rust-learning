struct People {
    name: String,
}

pub fn lifetime_simple_test() {
    println!(
        "{}",
        "------------lifetime_simple_test start-------------------"
    );

    lifetime_grammar_one();

    lifetime_grammar_two();

    lifetime_grammar_three();
}

fn lifetime_grammar_one() {
    //This will casue error
    /*
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
    */
    let x;
    let r;
    {
        x = 5;
        r = &x;
    }
    println!("r: {}", r);
}

fn lifetime_grammar_two() {
    /* This will cause error
    let mut x;
    let r;
    {
        x = &mut String::from("test");
        x.push_str("test");
        r = x
    }
    println!("{}",r);
    */
    let mut x;
    let r;
    {
        x = String::from("test");
        x.push_str("test");
        r = x
    }
    println!("{}", r);
}

fn lifetime_grammar_three() {
    let x;
    let r;
    {
        x = People {
            name: String::from("wenbin"),
        };
        r = &x;
    }
    println!("{}", r.name);

    /*
    let x;
    let r;
    {
        x = & People {
            name: String::from("wenbin"),
        };
        r = &x;
    }
    println!("{}", r.name);
    */
}

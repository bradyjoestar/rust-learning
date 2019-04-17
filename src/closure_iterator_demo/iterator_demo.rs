pub fn iterator_simple_test() {
    println!(
        "{}",
        "------------iterator_demo_test start-------------------"
    );

    let v1 = vec![1, 2, 3];
    //通过调用定义于 Vec 上的 iter 方法在一个 vector v1 上创建了一个迭代器
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let mut v2 = vec![1, 2, 3];
    {
        let mut v2_iter = v2.iter_mut(); //into_iter seems like move

        //    let mut v2_iter = v2.into_iter();  //iter_mut seems like &mut

        match v2_iter.next() {
            Some(v) => {
                println!("{}", v);
            }
            None => panic!(),
        };
    }
    println!("{}", v2.get(0).unwrap());
    println!("v2 length is {}", v2.len());

    let mut v3 = vec![1, 2, 3];
    {
        let mut v3_iter = v3.into_iter(); //into_iter seems like move

        //    let mut v2_iter = v2.into_iter();  //iter_mut seems like &mut

        match v3_iter.next() {
            Some(v) => {
                println!("{}", v);
            }
            None => panic!(),
        };
    }

    //This will cause error because
    /*
     let mut v3_iter = v3.into_iter;
                           value used here after move
    */
    //println!("{}",v3.get(0).unwrap());
    //println!("v3 length is {}",v3.len());
}

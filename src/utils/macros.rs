#[macro_export]
macro_rules! fmt(($token:expr) => (format!("{:?}", $token)));

#[macro_export]
macro_rules! say_hello{
   ()=>(
        println!("Hello");
   )
}
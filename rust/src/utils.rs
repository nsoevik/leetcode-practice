use std::fmt::Debug;

pub fn print<T>(label: &str, val: T)
where
    T: Debug,
{
    println!("{:?}: {:?}", label, val);
}

use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let mut vec = vec![];
    for i in 1u64..=num/2 {
        if &num % i == 0 && i != num{
            vec.push(i);
        }
    }

    let sum: u64 = vec.iter().sum();

    match sum.cmp(&num){
        Ordering::Less => Some(Classification::Deficient),
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Abundant),
    }
}

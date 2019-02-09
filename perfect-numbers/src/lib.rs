use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {

    let sum: u64 = (1..num/2 + 1) //numbers from 1 to num/2+1
        .filter(|f| num % f == 0) //filter out nonfactors
        .sum(); 

    match sum.cmp(&num){
        Ordering::Less => Some(Classification::Deficient),
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Abundant),
    }
}

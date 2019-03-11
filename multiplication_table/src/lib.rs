pub fn multip_table(n: usize) -> Vec<Vec<usize>> {
    let mut table = Vec::new();
    for y in 1..=n{
        let mut row = Vec::new();
        for x in 0..=n{
            row.push(x*y);
        }
        table.push(row);
    }
    return table;
}

pub fn multip_table_iter(n: usize) -> Vec<Vec<usize>> {
    (1..=n).map(|y|{
        (1..=n).map(|x| x * y)
               .collect()
    }).collect()
}

pub fn table_neatprint<T: std::string::ToString>(table: Vec<Vec<T>>) -> String{
    table.iter()
        .fold("".to_string(), |acc: String, row| {
            format!("{} \n {}", acc,
                    row.iter()
                    .fold("".to_string(), |rowacc, element: &T|{
                        format!("{}\t{}", rowacc, element.to_string())
                    }
                    )
                   )
        })
}

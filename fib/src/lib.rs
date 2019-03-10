pub fn fib(n: usize) -> usize{
    let mut fib_arr = vec![1, 1];

    for i in fib_arr.len()..n{
        fib_arr.push(fib_arr[i-1] + fib_arr[i-2]);
    }

    return fib_arr[n-1];
}

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
    (1..=n)
        .fold(Vec::new(), |mut arr: Vec<Vec<usize>>, y| {
            arr.push((1..=n)
                     .fold(Vec::new(), |mut row: Vec<usize>, x|{
                         row.push(x*y);
                         row
                     }
                     )
                    );
            arr
        }
        )


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

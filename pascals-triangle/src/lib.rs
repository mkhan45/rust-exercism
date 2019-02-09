pub struct PascalsTriangle {
    rows: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle{
            rows: row_count,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.rows == 1 {
            return vec![vec![1]];
        }
        let mut return_vec = vec![vec![1], vec![1, 1]];
        for y in 2..self.rows as usize {
            let temp_vec = {
                let mut holder = vec![1];
                for x in 1..(return_vec[y - 1].len()) {
                    if x <= return_vec[y - 1].len() - 1 {
                        holder.push(return_vec[y-1][x-1] + return_vec[y-1][x]);
                    } else {
                        holder.push(1);
                    }
                }
                holder.push(1);
                holder
            };
            return_vec.push(temp_vec);
        }
        return_vec
    }
}

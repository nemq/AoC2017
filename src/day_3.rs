    use std::f64;

    pub fn first_puzzle() -> String
    {
        format!("{}", steps(277678))
    }

    pub fn second_puzzle() -> String
    {
        let mut i = 1;
        let mut g = grid(i);
        while g <= 277678
        {
            i+= 1;
            g = grid(i)
        }

        format!("{}", g)
    }

    fn x(n: i64) -> i64
    {
        if n == 1
        {
            0
        }
        else
        {
            let mut seq = vec![0];
            let len = n as usize;
            for i in 1 .. len
            {
                let k = (f64::floor(f64::sqrt((4 * (i-1) + 1) as f64)) as i64) % 4;
                let el = seq[i-1] + f64::sin(k as f64 * f64::consts::PI / 2.0f64) as i64;
                seq.push(el);
            }

            seq[len-1]
        }
    }

    fn y(n: i64) -> i64
    {
        if n == 1
        {
            0
        }
        else
        {
            let mut seq = vec![0];
            let len = n as usize;
            for i in 1 .. len
            {
                let k = (f64::floor(f64::sqrt((4 * (i-1) + 1) as f64)) as i64) % 4;
                let el = seq[i-1] - f64::cos(k as f64 * f64::consts::PI / 2.0f64) as i64;
                seq.push(el);
            }

            seq[len-1]
        }
    }

    fn steps(n: i64) -> i64
    {
        let (x, y) = (x(n) , y(n));
        i64::abs(x) + i64::abs(y)
    }

    fn n(x_: i64, y_: i64) -> i64
    {
        let mut i = 1;
        loop
        {
            if x(i) == x_ && y(i) == y_
            {
                return i
            }
            i+= 1;
        }
    }

    fn adjacent_coord(x: i64, y: i64) -> Vec<(i64, i64)>
    {
        let mut adj = Vec::new();
        for dx in vec![-1,0, 1]
        {
            for dy in vec![-1, 0, 1]
            {
                if dx != 0 || dy != 0
                {
                    adj.push((x + dx, y +dy));
                }
            }
        }
        adj
    }

    fn adjacent_sum(x: i64, y: i64, grid: &Vec<i64>) -> i64
    {
        let mut sum = 0;
        for (a_x, a_y) in adjacent_coord(x, y)
        {
            let idx  = n(a_x, a_y) - 1;
            if idx < grid.len() as i64
            {
                sum += grid[idx as usize];
            }
        }
        sum
    }

    fn grid(n :i64) -> i64
    {
        if n == 1
        {
            1
        }
        else
        {
            let mut grid = vec![1];
            for i in 2 .. (n+1) as usize
            {
                let sum = adjacent_sum(x(i as i64), y(i as i64), &grid);
                grid.push(sum);
            }

            grid[grid.len() - 1]
        }
    }

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn first_puzzle()
    {
        assert_eq!(0, x(1));
        assert_eq!(1, x(9));
        assert_eq!(2, x(12));
        assert_eq!(0, x(15));
        assert_eq!(0, x(23));


        assert_eq!(0, y(1));
        assert_eq!(-1, y(9));
        assert_eq!(1,  y(12));
        assert_eq!(2, y(15));
        assert_eq!(-2, y(23));

        assert_eq!(0, steps(1));
        assert_eq!(2, steps(9));
        assert_eq!(3, steps(12));
        assert_eq!(2, steps(15));
        assert_eq!(2, steps(23));
    }

    #[test]
    fn second_puzzle()
    {
        assert_eq!(1, n(0, 0));
        assert_eq!(9, n(1, -1));
        assert_eq!(12, n(2, 1));
        assert_eq!(15, n(0, 2));
        assert_eq!(23, n(0, -2));

        assert_eq!(1, grid(1));
        assert_eq!(1, grid(2));
        assert_eq!(2, grid(3));
        assert_eq!(806, grid(23));
    }
}
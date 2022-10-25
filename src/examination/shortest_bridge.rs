#[allow(dead_code)]
pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
    // 找到第一块陆地
    let (mut x, mut y) = (0,0);
    'first: for (row, row_value) in grid.iter().enumerate() {
        for (col, value) in row_value.iter().enumerate() {
            if *value == 1 {
                x = row;
                y = col;
                break 'first;
            }
        }
    };
    grid[x][y] = 2;
    let mut res = Vec::default();
    res.push((x, y));
    find(&mut grid, &mut res, (x, y));
    print!("{:?}", &grid);

    find2(&mut grid, res, 0)
}

fn find2(grid: &mut Vec<Vec<i32>>, res: Vec<(usize, usize)>, count: i32) -> i32 {
    let mut ext = Vec::default();
    for (x, y) in res {
        if x > 0 {
            match grid[x - 1][y] {
                0 => {            
                    grid[x - 1][y] = 2;
                    ext.push((x - 1, y))
                },
                1 => return count,
                _ => (),
            }
        }
        if y > 0 {
            match grid[x][y - 1] {
                0 => {            
                    grid[x][y - 1] = 2;
                    ext.push((x, y - 1))
                },
                1 => return count,
                _ => (),
            }
        }
        if x < grid.len() - 1 {
            match grid[x + 1][y] {
                0 => {            
                    grid[x + 1][y] = 2;
                    ext.push((x + 1, y))
                },
                1 => return count,
                _ => (),
            }
        }
        if y < grid.len() -1 {
            match grid[x][y + 1] {
                0 => {            
                    grid[x][y + 1] = 2;
                    ext.push((x, y + 1))
                },
                1 => return count,
                _ => (),
            }
        }
    }
    find2(grid, ext, count + 1)
}

fn find(grid: &mut Vec<Vec<i32>>, res: &mut Vec<(usize, usize)>, (x, y): (usize, usize)) {
    if x > 0 && grid[x - 1][y] == 1 {
        grid[x - 1][y] = 2;
        res.push((x - 1, y));
        find(grid, res, (x - 1, y));
    }
    if y > 0 && grid[x][y - 1] == 1 {
        grid[x][y - 1] = 2;
        res.push((x, y - 1));
        find(grid, res, (x, y - 1));
    }
    if x < grid.len() - 1 && grid[x + 1][y] == 1 {
        grid[x + 1][y] = 2;
        res.push((x + 1, y));
        find(grid, res, (x + 1, y));
    }
    if y < grid.len() - 1 && grid[x][y + 1] == 1 {
        grid[x][y + 1] = 2;
        res.push((x, y + 1));
        find(grid, res, (x, y + 1));
    }
}


#[cfg(test)]
mod shortest_bridge_test {
    use super::*;

    #[test]
    fn shortest_bridge_test() {
        assert_eq!(shortest_bridge(vec![
            vec![0,1,1,1,0], 
            vec![0,1,0,1,0], 
            vec![0,1,0,0,0], 
            vec![0,0,0,0,1]]), 2);
    }
}
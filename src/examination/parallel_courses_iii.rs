use std::{cell::RefCell, rc::Rc, collections::hash_set::HashSet};

#[allow(dead_code)]
pub fn minimum_time(_n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
    let mut hash_set = HashSet::new();
    let mut root = Vec::new();
    for (i, ele) in time.iter().enumerate() {
        let node = Rc::new(RefCell::new(Node {
            _index: i + 1,
            time: *ele,
            next: Vec::new(),
        }));
        root.push(node);
    }
    for relation in relations {
        let left_node = root.get(relation[0] as usize - 1).unwrap();
        let right_node = root.get(relation[1] as usize - 1).unwrap();
        hash_set.insert(relation[1]);
        left_node.borrow_mut().next.push(Rc::clone(right_node));
    }
    println!("{:#?}", root);
    let mut max = 0;
    resolve(&root, &mut max, 0, &hash_set);
    max
}

fn resolve(list: &Vec<Rc<RefCell<Node>>>, max: &mut i32, current: i32, hash_set: &HashSet<i32>) {
    for (i, ele) in list.iter().enumerate() {
        if current == 0 && hash_set.contains(&(i as i32 + 1)) {
            continue;
        }
        let tmp = current + ele.borrow().time;
        if *max < tmp {
            *max = tmp;
        }

        resolve(&ele.borrow().next, max, tmp, hash_set);
    }
}

#[derive(Debug)]
struct Node {
    pub _index: usize,
    pub time: i32,
    pub next: Vec<Rc<RefCell<Node>>>,
}

#[cfg(test)]
mod parallel_courses_iii_test {
    use super::*;

    #[test]
    fn minimum_time_test() {
        let relations = vec![vec![1,3],vec![2,3]];
        let time= vec![3,2,5];
        assert_eq!(minimum_time(3,relations,time), 8);
        
        let relations = vec![vec![1,5],vec![2,5],vec![3,5],vec![3,4],vec![4,5]];
        let time= vec![1,2,3,4,5];
        assert_eq!(minimum_time(5,relations,time), 12);
        
        let relations = vec![];
        let time= vec![1];
        assert_eq!(minimum_time(1,relations,time), 1);
        
        let relations = vec![];
        let time= vec![3,5];
        assert_eq!(minimum_time(2,relations,time), 5);
    }
}
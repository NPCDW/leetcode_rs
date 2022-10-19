#[allow(dead_code)]
pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut students = students;
    'sandwiche_for: for sandwiche in sandwiches {
        let mut count = 0;
        loop {
            if count == students.len() {
                return students.len().try_into().unwrap();
            }
            if students.get(0).unwrap() == &sandwiche {
                let _ = students.remove(0);
                continue 'sandwiche_for;
            }
            let value = students.remove(0);
            students.push(value);
            count += 1;
        }
    }
    return 0;
}
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Department {
    Engineering,
    Sales,
}

impl Department {
    fn init() -> HashMap<Department, Vec<String>> {
        let mut department = HashMap::new();
        use Department::*;
        let list = [Engineering, Sales];
        list.iter().for_each(|d| {
            let v: Vec<String> = vec![];
            department.entry(*d).or_insert(v);
        });
        department
    }
}

fn add_to_department(
    department: &mut HashMap<Department, Vec<String>>,
    name: &str,
    dept: Department,
) {
    department
        .entry(dept)
        .or_insert(vec![])
        .push(name.to_string());
}

fn retrieve_emp_from_department(
    department: &HashMap<Department, Vec<String>>,
    dept: Department,
) -> Vec<String> {
    department.get(&dept).unwrap_or(&vec![]).to_vec()
}

fn get_median(numbers: &[i32]) -> i32 {
    let mut numbers = numbers.to_vec();
    numbers.sort(); // Vector can be sorted
    numbers[numbers.len() / 2]
}

fn get_mode(number: &[i32]) -> i32 {
    let mut modes = HashMap::new();
    for n in number {
        let count = modes.entry(*n).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut max = 0;
    for (n, count) in modes {
        if count > max {
            max = count;
            mode = n;
        }
    }
    mode
}

fn pig_latin(word: &str) -> String {
    let first = word.chars().next().unwrap();
    match first {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", word.replace(first, ""), first),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_department() {
        let department = Department::init();
        assert_eq!(department.len(), 2);
        let v: Vec<String> = vec![];
        use Department::*;
        assert_eq!(department[&Engineering], v);
        assert_eq!(department[&Sales], v);
    }

    #[test]
    fn test_add_to_department() {
        use Department::*;
        let mut department = Department::init();
        let v: Vec<String> = vec![];
        assert_eq!(*department.get(&Sales).unwrap(), v);
        add_to_department(&mut department, "Sally", Sales);
        assert_eq!(*department.get(&Sales).unwrap(), vec!["Sally"]);
        add_to_department(&mut department, "Amir", Engineering);
        assert_eq!(*department.get(&Engineering).unwrap(), vec!["Amir"]);
    }

    #[test]
    fn test_retrieve_department() {
        use Department::*;
        let mut department = Department::init();
        let v: Vec<String> = vec![];
        assert_eq!(retrieve_emp_from_department(&department, Engineering), v);
        add_to_department(&mut department, "Sally", Sales);
        add_to_department(&mut department, "Amir", Engineering);
        add_to_department(&mut department, "Tom", Engineering);
        assert_eq!(
            retrieve_emp_from_department(&department, Engineering),
            vec!["Amir", "Tom"]
        );
    }

    #[test]
    fn test_get_median() {
        let numbers = [1, 2, 3, 4, 5];
        let median = get_median(&numbers);
        assert_eq!(median, 3);
    }

    #[test]
    fn test_get_mode() {
        let numbers = [1, 2, 4, 4, 5];
        let mode = get_mode(&numbers);
        assert_eq!(mode, 4);
    }

    #[test]
    fn test_pig_latin() {
        let first = "first";
        assert_eq!("irst-fay", pig_latin(first));
        let apple = "apple";
        assert_eq!("apple-hay", pig_latin(apple));
    }
}

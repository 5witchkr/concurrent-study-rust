
fn main() {

    // ex_1();
     ex_2();
    // ex_3();
    // ex_4();
}



fn ex_1() {
    let num = 5;
    let add_num = |x: i32| x + num;

    println!("result: {}", add_num(10));
    println!("result: {}", add_num(3));
}

fn ex_2() {
    fn create_adder(a: i32) -> impl Fn(i32) -> i32 {
        move |b| a + b
    }
    
    let add_5 = create_adder(5);
    let add_10 = create_adder(10);
    println!("{}", add_5(10));
    println!("{}", add_5(3));
    println!("--------");
    println!("{}", add_10(10));
    println!("{}", add_10(3));
}

fn ex_3() {
    fn sort_numbers(numbers: &Vec<i32>, compare: impl Fn(&i32, &i32) -> std::cmp::Ordering) -> Vec<i32> {
        let mut cloned_numbers = numbers.clone();
        cloned_numbers.sort_by(compare);
        cloned_numbers
    }

    let numbers = vec![4, 2, 7, 5, 1, 3, 6];

    let ascending = |a: &i32, b: &i32| a.cmp(b);

    let descending = |a: &i32, b: &i32| b.cmp(a);

    let sorted_ascending = sort_numbers(&numbers, ascending);
    println!("Ascending order: {:?}", sorted_ascending);

    let sorted_descending = sort_numbers(&numbers, descending);
    println!("Descending order: {:?}", sorted_descending);
}

fn ex_4() {
    fn filter_numbers(numbers: &Vec<i32>, predicate: impl Fn(&i32) -> bool) -> Vec<i32> {
        numbers.iter().cloned().filter(predicate).collect()
    }

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even_filter = |x: &i32| x % 2 == 0;

    let odd_filter = |x: &i32| x % 2 != 0;

    let even_numbers = filter_numbers(&numbers, even_filter);
    println!("Even numbers: {:?}", even_numbers);

    let odd_numbers = filter_numbers(&numbers, odd_filter);
    println!("Odd numbers: {:?}", odd_numbers);
}

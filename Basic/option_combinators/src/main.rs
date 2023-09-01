fn print_type_of<T>(_:&T) {
    println!("Type: {:?}", std::any::type_name::<T>())
}


fn main() {
    let a: Option<i32> = None;
    let a_is_some = a.is_some();
    println!("{:?}", a_is_some);
    let a_is_noe = a.is_none();
    dbg!(a_is_noe);
    let a_mapped = a.map(|num| num + 2);
    dbg!(a_mapped);
    let a_filtered = a.filter(|num| num == &2);
    dbg!(a_filtered);
    let a_or_else = a.or_else(|| Some(11));
    dbg!(a_or_else);
    let unwrapped = a.unwrap_or_else(|| 0);
    dbg!(unwrapped);

    // iterater
    let numbers = vec![1, 3 ,4, 6, 8, 11];
    let mut plus_one = vec![];
    for num in &numbers {
        plus_one.push(num + 1);
    }
    dbg!(plus_one);
    let plus_two: Vec<_> = numbers
        .iter()
        .map(|num| num / 2)
        .collect();
    dbg!(&plus_two);
    print_type_of(&plus_two);
    println!(" count: {:?}", &numbers.iter().count());
    // mit ** dereference 
    println!("find {:?}", &numbers.iter().find(|num| **num == 11));
    // or && 
    println!("find {:?}", &numbers.iter().find(|num| num == &&11));
}

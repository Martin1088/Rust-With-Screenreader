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
}

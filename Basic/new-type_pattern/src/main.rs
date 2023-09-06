
#[derive(Debug)]
struct NeverZero(i32);
impl NeverZero{
    fn new(i: i32) -> Result<Self, String> {
        match i {
            0 => Err("cannot be Zero".to_owned()),
            _ => Ok(Self(i))
        }
    }
}

fn divide(a: i32, b: NeverZero) -> i32 {
    let b = b.0;
    a/b
}
fn main() {
    match NeverZero::new(0) {
        Ok(nz) => println!("Result: {:?}", divide(10, nz)),
        Err(e) => println!("Erro: {:?}", e),
    }
    }

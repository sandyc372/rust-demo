#![allow(unused)]
fn main() {

struct Data {
    value: u32
}

let mut optional = Some(Box::new(Data {
    value: 32
}));
check_optional(&mut optional);
match optional {
        Some(ref mut d) => {
        d.value = 693;
        println!("has value {}", d.value);
            
        },
        None => println!("has no value"),
    }

fn check_optional(optional: &mut Option<Box<Data>>) {
    match optional {
        Some(ref mut d) => {
        d.value = 63;
        println!("has value {}", d.value);
            
        },
        None => println!("has no value"),
    }
}
}
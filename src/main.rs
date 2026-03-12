use nepal_phone::{parse};

fn main() {
  let map = parse("014231481").unwrap().to_map();
  println!("{:?}", map);
}
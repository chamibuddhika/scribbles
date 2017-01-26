
struct Point {
     x: i32,
     y: i32,
}

fn primitive_move(mut x: i32) {
  x = 3;
}

fn struct_move(mut p: Point) {
  p.x = 5;
}

fn primitive_borrow(x: &mut i32) {
  *x = 3;
}

fn struct_borrow(p: &mut Point) {
  p.x = 2;
}

fn vec_mutability() {
  let mut v = vec![Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }];
  v.push(Point { x: 0, y: 0 });

  // let y = &mut v;

  for p in &mut v {
    p.x = 1;
    println!("{} {}", p.x, p.y);
    //   v.push(Point{x:0, y:0}); // A mutable borrow happens here
  }
}

fn ownership() {
  let x = 5;
  let y = x;
  println!("{}", x); // OK. Since i32 implement Copy trait

  let p = Point { x: 0, y: 0 };
  let q = p;
  // println!("{}", p.x); // Wrong. Since Point doesn't implement Copy trait

  let x1 = 5;
  let mut x2 = x1; // OK.
  x2 = 4;
  println!("{}", x1); // x1 is unchanged since it has its own copy

  let x3 = 2;
  let mut x4 = &x3;
  // *x4 = 5; // Wrong. Since this was an immutable borrow

  let x5 = 2;
  // let mut x6 = &mut x5; // Wrong. Cannot borrow immutable local variable mutably
  // *x6 = 5;

  let mut x7 = 5;
  let mut x8 = &mut x7;
  *x8 = 2; // OK. Mutating a mutably borrowed mutable variable

  let z = 4;
  primitive_move(z);
  println!("{}", z); // OK. Since i32 implements Copy trait

  let r = Point { x: 0, y: 0 };
  struct_move(r);
  // println!("{}", r.x); // Wrong. Since r was moved.

  let mut z1 = 4;
  primitive_borrow(&mut z1);
  println!("{}", z1); // OK. z1 is 3 now.

  let mut r1 = Point { x: 0, y: 0 };
  struct_borrow(&mut r1);
  println!("{}", r1.x); // OK. Borrowing is mutable


}

fn skip_prefix(line: &Point, prefix: &mut String) -> i32 {
  let mut x = &mut "abcd".to_string();
  x = prefix;
  println!("{}", x);
  2
    // ...
}

fn tmp() {
  let line = "lang:en=Hello World!";
  let mut lang = "en";

  let x = Point { x: 0, y: 0 };

  let v;
  {
    let p = format!("lang:{}=", lang); // -+ p goes into scope
    v = skip_prefix(&x, &mut lang.to_string()); //  |
  }
}

fn main() {
  vec_mutability();
  ownership();
  tmp();
} 

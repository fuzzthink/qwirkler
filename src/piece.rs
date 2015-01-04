pub use self::Colour::{R, O, Y, G, B, P};
pub use self::Shape::{H,I,J,K,L,M};

#[derive(Show, Clone)]
pub struct Piece(Colour, Shape);


#[derive(Show, Clone)]
pub enum Colour {
  R = 0, O = 1, Y = 2, G = 3, B = 4, P = 5,
}

impl Colour {
  fn all() -> Vec<Colour> {
    vec![R,O,Y,G,B,P]
  }
}

// TODO: define default types or something http://doc.rust-lang.org/std/default/
// TODO: equality of members?
// TODO: make these enums not involve pointers.

#[derive(Show, Clone)]
pub enum Shape {
  H=10,I=20,J=30,K=40,L=50,M=60,
}

impl Shape {
  fn all() -> Vec<Shape> {
    vec![H,I,J,K,L,M]
  }
}

fn combinations() -> Vec<Piece> {
  let mut pieces = Vec::with_capacity(36);

  for colour in Colour::all().iter() {
    for shape in Shape::all().iter() {
      pieces.push(Piece(colour.clone(), shape.clone())) // TODO bad cloning
    }
  }

  return pieces
}

pub fn make_bag() -> Vec<Piece> {
  let mut full_bag = Vec::with_capacity(108);
  let pieces = combinations();

  for i in [1,2,3].iter() {
    full_bag.push_all(pieces.as_slice())
  }

  return full_bag
}
use std::ops::BitAnd;

enum Bool { True, False }
enum Nat { Zero, Suc (Box<Nat>)  }

use Bool::*;
use Nat::*;

impl BitAnd for Bool {
    type Output = Bool;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (True, True) => True,
            (_, _)       => False
        }
    }
}

trait Eq {
  fn eq(&self, rhs: &Self) -> Bool;
}

impl Eq for Nat {
  fn eq(&self, rhs: &Self) -> Bool {
    match (self, rhs) {
      (Zero  , Zero    ) => True,
      (Suc(x), Suc(y))   => x.eq(y),
      (_, _)        => False
    }
  }
}

impl<A: Eq> Eq for [A] {
  fn eq(&self, rhs: &Self) -> Bool {
    match (self, rhs) {
      ([], []) => True,
      ([x, xs @ ..], [y, ys @ ..]) 
        => x.eq(y) & xs.eq(ys),
      (_ , _)  => False
    }
  }
}

fn isEq() -> Bool { [Zero].eq(&[Zero]) }
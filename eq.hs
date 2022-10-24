module Eq where

import qualified Prelude

infixl 1 &&
(&&) :: Bool -> Bool -> Bool
True && True = True
_ && _ = False

data Bool = True | False
data Nat  = Zero | Suc Nat

class Eq α where
  eq :: α -> α -> Bool

instance Eq Nat where
  eq Zero    Zero    = True
  eq (Suc x) (Suc y) = eq x y
  eq _        _      = False

instance Eq α => Eq [α] where
  eq []       []       = True
  eq (x : xs) (y : ys) = eq x y && eq xs ys
  eq _           _     = False

isEq :: Bool
isEq = eq [Zero] [Zero]
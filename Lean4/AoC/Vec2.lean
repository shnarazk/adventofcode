import Lean

namespace Vec2

open Lean
open Lean.Parser

abbrev Vec2 := Int64 × Int64

instance : ToString Vec2 where toString v := s!"({v.1},{v.2})"
instance : Hashable Int64 where hash a := a.toUInt64
-- instance : Hashable Vec2 where hash a := hash (a.1)o

instance : HAdd Vec2 Vec2 Vec2 where
  hAdd (a b : Vec2) : Vec2 := (a.1 + b.1, a.2 + b.2)

instance : HAdd Vec2 Int64 Vec2 where
  hAdd (v : Vec2) (a : Int64) : Vec2 := (v.1 + a, v.2 + a)

instance : HSub Vec2 Vec2 Vec2 where
  hSub (a b : Vec2) : Vec2 := (a.1 - b.1, a.2 - b.2)

instance : HSub Vec2 Int64 Vec2 where
  hSub (v : Vec2) (a : Int64) : Vec2 := (v.1 - a, v.2 - a)

instance : LT Vec2 where
  lt (a b : Vec2) := a.1 < b.1 ∧ a.2 < b.2

instance instDecidableLtVec2 (a b : Vec2) : Decidable (a < b) := by
  dsimp [instLTVec2]
  exact instDecidableAnd

-- #eval ((0, 0) : Vec2) < ((8, 2) : Vec2)

instance : LE Vec2 where
  le (a b : Vec2) := a.1 ≤ b.1 ∧ a.2 ≤ b.2

instance instDecidableLeVec2 (a b : Vec2) : Decidable (a ≤ b) := by
  dsimp [instLEVec2]
  exact instDecidableAnd

-- #eval ((0, 0) : Vec2) ≤ ((8, 2) : Vec2)

-- def contains (size pos : Vec2) : Bool :=
--   0 ≤ pos.1 && pos.1 < size.1 && 0 ≤ pos.2 && pos.2 < size.2

def geZeroAndLe (size pos : Vec2) : Bool := (0, 0) ≤ pos && pos ≤ size

syntax:50 term:51 " ≤₀ " term:50 : term
macro_rules | `($a ≤₀ $b) => `(geZeroAndLe $b $a)

-- #eval ((0, 0) : Vec2) ≤ (3, 2)
-- #eval geZeroAndLe (5, 5) (3, 2)
-- #eval (3, 2) ≤₀ (5, 5)

def geZeroAndLt (size pos : Vec2) : Bool := (0, 0) ≤ pos && pos < size

syntax:50 (name := syntaxInfixGeZeroAndLt) term:51 " <₀ " term:50 : term
macro_rules | `($a <₀ $b) => `(geZeroAndLt $b $a)

-- @[macro syntaxInfixGeZeroAndLt]
-- def infixGeZeroiAndLt : Macro :=
-- fun stx => match stx with
-- |  `($a <₀ $b) => `(geZeroAndLt $b $a)
-- | _ => Macro.throwUnsupported


-- #eval ((0, 0) : Vec2) < (3, 2)
-- #eval geZeroAndLt (5, 5) (3, 2)
-- #eval (3, 2) <₀ (5, 5)

end Vec2

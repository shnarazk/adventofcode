import Lean
import Aesop

/--
Build and return a data filename
-/

def dataFileName (year day : Nat) (ext : Option String): IO String := do
  let aoc_dir ← IO.getEnv "AOC_DIR"
  let d := ("0" ++ s!"{day}").takeRight 2
  return match ext with
  | some ext => s!"{aoc_dir.getD ".."}/data/{year}/input-day{d}-{ext}.txt"
  | none     => s!"{aoc_dir.getD ".."}/data/{year}/input-day{d}.txt"


def readData (datafilename : String) : IO String := do
   IO.FS.readFile datafilename

-- #eval dataFileName 2023 2 none

/--
return file contents as String
-/
def dataOf (year day : Nat) (ext : Option String): IO String :=
  dataFileName year day ext >>= readData

def readLines (datafilename : String) : IO (Array String) := do
     IO.FS.lines datafilename

/--
return file contents as Array String
-/
def linesOf (year day : Nat) (ext : Option String): IO (Array String) :=
  dataFileName year day ext >>= readLines

-- end FileIO

def Answers := String × String

structure AocProblem where
  year : Nat
  day : Nat
  validYear : 2000 < year
  validDay : 1 ≤ day ∧ day ≤ 25
  input_name : String
  answers: Option (String × String) := none
  time: Float := 0
deriving BEq, Repr
instance : ToString AocProblem where toString s := s!"Y{s.year}D{s.day}"

--#check AocProblem.mk 2024 8 (by simp)

namespace AocProblem
def new (year day : Nat) : AocProblem :=
  have valid_year : 2000 < max year 2001 := by
    have : 2001 ≤ max year 2001 := by exact Nat.le_max_right year 2001
    exact this
  have valid_day : 1 ≤ min (max day 1) 25 ∧ min (max day 1) 25 ≤ 25 := by
    constructor
    {
      have : 1 ≤ max day 1 := by exact Nat.le_max_right day 1
      have H : 1 ≤ 25 := by exact Nat.lt_of_sub_eq_succ rfl
      have : 1 ≤ min (max day 1) 25 := by exact Nat.le_min_of_le_of_le this H
      exact this
    }
    { exact Nat.min_le_right (max day 1) 25 }
  AocProblem.mk
    (max year 2001)
    (min (max day 1) 25)
    valid_year
    valid_day
    ""
    none
    0

def fileName (self : AocProblem) (ext : Option String) : IO String :=
  dataFileName self.year self.day ext

def getData (self : AocProblem) (ext : Option String) : IO String :=
  dataFileName self.year self.day ext >>= readData

def getLines (self : AocProblem) (ext : Option String) : IO (Array String) :=
  dataFileName self.year self.day ext >>= readLines

instance : Lean.ToJson AocProblem where
  toJson (s: AocProblem) :=
    Lean.Json.mkObj $ ⟨"AoCProblem", "0.1"⟩ :: [
      ⟨"year", Lean.ToJson.toJson s.year⟩,
      ⟨"day", Lean.ToJson.toJson s.day⟩,
      ⟨"time", Lean.ToJson.toJson s.time⟩,
    ]

-- #eval Lean.ToJson.toJson (AocProblem.new 2024 10)

def toJson (self : AocProblem) : Lean.Json := Lean.ToJson.toJson self

def build {α β γ : Type} [ToString β] [ToString γ]
    (self : AocProblem)
    (parser : String → Option α)
    (solve₁ : α → β) (solve₂ : α → γ)
    (alt : Option String)
    : IO AocProblem := do
  if let some d := parser (← self.getData alt)
  then return { self with
    input_name := (← self.fileName alt)
    answers := some (s!"{solve₁ d}", s!"{solve₂ d}") }
  else return { self with
    input_name := (← self.fileName alt)
    answers := none }

end AocProblem

/--
Return an array consisting of elements in `a`
-/
def unique (a : Array α) [BEq α] [Hashable α] : Array α :=
  let hash := Array.foldl (·.insert ·) (Std.HashSet.empty : Std.HashSet α) a
  hash.toArray

-- #eval unique #[1, 0, 2, 1, 3, 2]

class Accumulation (α : Type) where
  sum    : α → Nat
  product: α → Nat

instance : Accumulation (List Nat) where
  sum     a := a.foldl Nat.add 0
  product a := a.foldl Nat.mul 1

instance : Accumulation (List Int) where
  sum     a := Int.toNat <| a.foldl Int.add 0
  product a := Int.toNat <| a.foldl Int.mul 1

instance : Accumulation (Array Nat) where
  sum     a := a.foldl Nat.add 0
  product a := a.foldl Nat.mul 1

instance : Accumulation (Array Int) where
  sum     a := Int.toNat <| a.foldl Int.add 0
  product a := Int.toNat <| a.foldl Int.mul 1

open Accumulation

-- #eval Accumulation.sum [1, 3, 5]
-- #eval sum [1, 3, 5]
-- #eval product [1, 3, 5]

def List.enumerate (a : List α) : List (Nat × α) := List.zip (List.range a.length) a

-- #eval [2, 4, 5].enumerate

def Array.enumerate (a : Array α) : Array (Nat × α) := Array.zip (Array.range a.size) a

-- #eval #[2, 4, 5].enumerate

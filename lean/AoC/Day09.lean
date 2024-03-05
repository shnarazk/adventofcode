import Std
import Lean.Data.Parsec
import «AoC».Basic
import «AoC».Parser

namespace Day09
open Std

-- structure Data where
--deriving Repr

namespace parser
open Lean.Parsec AoCParser

def parser := sepBy1 (sepBy1 number_signed (pchar ' ')) eol

end parser

def windows₂ (l : List α) : List (α × α) :=
  match l with
  | []          => []
  | a :: b :: c => (a, b) :: windows₂ (b :: c)
  | [_]         => []

namespace part1

def evaluate_ (n : Nat) (a : List Int) : Int :=
  -- n is used for termination assertion
  -- or proove diff length is smaller than a's
  match n with
  | 0 => 0
  | n' + 1 =>
    let diff : List Int := windows₂ a |>.map (fun (a, b) => b - a)
    if diff.all (· == 0)
    then a.getLast!
    else (evaluate_ n' diff) + a.getLast!

def evaluate (a : Array Int) : Int := evaluate_ a.size a.toList

def solve (data : String) : IO Unit := do
  match AoCParser.parse parser.parser data with
  | none   => IO.println s!"  part1: parse error"
  | some d => IO.println s!"  part1: {d.map evaluate |>.foldl (. + .) 0}"
  return ()

end part1

-- #eval solve2_line ""
namespace part2

def evaluate_ (n : Nat) (a : List Int) : Int :=
  -- n is used for termination assertion
  -- or proove diff length is smaller than a's
  match n with
  | 0 => 0
  | n' + 1 =>
    let diff : List Int := windows₂ a |>.map (fun (a, b) => b - a)
    if diff.all (· == 0)
    then a.getLast!
    else a[0]! - (evaluate_ n' diff)

def evaluate (a : Array Int) : Int := evaluate_ a.size a.toList

def solve (data : String) : IO Unit := do
  match AoCParser.parse parser.parser data with
  | none   => IO.println s!"  part2: parse error"
  | some d => IO.println s!"  part2: {d.map evaluate |>.foldl (. + .) 0}"
  return ()

end part2

end Day09

def day09 (ext : Option String) : IO Unit := do
  let data ← dataOf 2023 9 ext
  Day09.part1.solve data
  Day09.part2.solve data

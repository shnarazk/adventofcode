import «AoC».Basic
import «AoC».Combinator
import «AoC».Parser
import «AoC».Rect64

namespace Y2024.Day08

open Std Accumulation CiCL TwoDimensionalVector64

structure Input where
  line : Array (Array Char)
  anntena : Array (Dim2 × Char)
deriving BEq, Hashable, Repr
-- #check ((4, 8) : Dim2)

instance : ToString Input where toString _ := s!""

namespace parser

open AoCParser
open Std.Internal.Parsec
open Std.Internal.Parsec.String

def parse : String → Option Input := AoCParser.parse parser
  where
    parser : Parser Input := do
      let v ← sepBy1 alphabets eol
      return Input.mk (v.map (·.toList.toArray)) Array.empty

end parser

namespace Part1

def solve (input : Input) : Nat :=
  input.anntena
    |>.foldl
        (fun (h : HashSet Dim2) _anntena ↦ h)
        HashSet.empty
    |>.size

end Part1

namespace Part2

def solve (_ : Input) : Nat := 0

end Part2

def solve := AocProblem.config 2024 08
  ((dbg "parsed as ") ∘ parser.parse)
  Part1.solve
  Part2.solve

end Y2024.Day08
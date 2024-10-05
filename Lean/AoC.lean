-- This module serves as the root of the `AoC` library.
-- Import modules here that should be built as part of the library.
-- import Mathlib.Data.Rat.Basic
-- import Mathlib.Data.Real.Basic
import Aesop
import Batteries
import «AoC».Combinator
import «AoC2023»
import «AoC2024»

def run (year: Nat) (day : Nat) (extra : Option String) : IO Unit := do
  let f ← dataFileName year day extra
  let ans ← match year with
    | 2023 =>
      if h : day - 1 < AoC2023.solvedDays
        then
          let ans ← Aesop.time <| AoC2023.solve day h extra
          pure (some (ans))
        else
          do pure none
    | 2024 =>
      if h : day - 1 < AoC2024.solvedDays
        then
          let res ← Aesop.time <| AoC2024.solve day h extra
          pure (some (res))
        else
          do pure none
    | _ => do pure none
  match ans with
    | some (results, time) => do
      IO.println s!"{color.blue}- {f}{color.reset}: {time.printAsMillis}{color.reset}"
      IO.println s!"{color.green}  => {results.fst}, {results.snd}{color.reset}"
    | _ => do return

def aoc_driver (args : List String) : IO Unit := do
  let extra := args.get? 2
  match (args.get? 0).map (·.toNat!) with
  | some year => do
      let solved := match year with
        | 2023 => List.range AoC2023.solvedDays
        | 2024 => List.range AoC2024.solvedDays
        | _ => []
      match (args.get? 1).map (·.toNat!) with
        | some day => run year day extra
        | none     => let _ ← do (solved.mapM (run year · extra))
  | none => return

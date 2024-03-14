import Std
import Lean.Data.Parsec
import «AoC».Basic
import «AoC».Combinator
import «AoC».Parser

open Accumulation
namespace Day12
open Std CoP

structure Data where
  new ::
  pattern : Array Char
  rule    : Array Nat
deriving Repr

instance : ToString Data where
  toString s := s!"\"{String.intercalate "" (Array.map toString s.pattern).toList}\" :: {s.rule}\n"

namespace parser
open Lean.Parsec AoCParser

def line_parser := do
  let pattern ← many1 (pchar '.' <|> pchar '#' <|> pchar '?') <* whitespaces
  let rule    ← sepBy1 number (pchar ',')
  return Data.new pattern rule

def parser := sepBy1 line_parser eol

end parser

#eval [1, 2, 5].tail
#eval List.drop 2 [1, 2, 5]
#eval compare 3 5 == Ordering.lt

def match_sequence
    (hash   : HashMap (String × Nat) Nat)
    (limit : Nat)
    (target : List Char)
    (rule   : List Nat)
    : (HashMap (String × Nat) Nat) × Nat :=
  -- check memorized value first
  match limit with
  | 0 => (hash, 0)
  | limit' + 1 =>
    let key := (target.foldl (fun s e => s.push e) "", rule.length)
    match hash.find? key with
    | some combinations => (hash, combinations)
    | none =>
      let key := (target.foldl (fun s e => s.push e) "", rule.length)
      match target with
      | []      => let x := if rule == [] then 1 else 0 ;  (hash.insert key x, x)
      | t :: t' =>
        match rule with
        | []      => let x := if target.all (· != '#') then 1 else 0 ; (hash.insert key x, x)
        | r :: r' =>
          -- index boundary check
          if target.length < rule.length then (hash.insert key 0, 0) else
            match t with
            | '.' => match_sequence hash limit' t' rule
            | '#' =>
              let chank_len : Nat := target.findIdx (· != t)
              match compare r chank_len with
              | Ordering.lt => (hash.insert key 0, 0)
              | Ordering.eq => match_sequence hash limit' (List.drop r t') r'
              | Ordering.gt =>
                if target.length < r
                then (hash.insert key 0, 0)
                else
                  if (List.range r).all (target[·]! != '.') && (r == target.length || target[r]! != '#')
                  then match_sequence hash limit' (List.drop r t') r'
                  else (hash.insert key 0, 0)
            | '?' =>
              let (h', m) := match_sequence hash limit' ('.' :: t') rule
              let (h'', n) := match_sequence h' limit' ('#' :: t') rule
              (h''.insert key (m + n), m + n)
            | _   => panic "impossible"
termination_by target.length + limit

-- #eval match_sequence HashMap.empty "..".toList [] |>.snd
-- #eval match_sequence HashMap.empty "##".toList [2] |>.snd
-- #eval match_sequence HashMap.empty "#.#".toList [1,1] |>.snd
-- #eval match_sequence HashMap.empty 100 "?".toList [1] |>.snd
-- #eval match_sequence HashMap.empty 100 "??".toList [2] |>.snd
-- #eval match_sequence HashMap.empty 100 "#?".toList [2] |>.snd
-- #eval match_sequence HashMap.empty 100 "?#?".toList [2] |>.snd
-- #eval match_sequence HashMap.empty 100 "#?#?".toList [2] |>.snd
-- #eval match_sequence HashMap.empty 100 "#?#?".toList [3] |>.snd
-- #eval match_sequence HashMap.empty 100 "#?#?".toList [4] |>.snd
-- #eval match_sequence HashMap.empty 100 "?#?#?".toList [3] |>.snd

def Part1.evaluate (conf : Data) : Nat :=
  match_sequence (HashMap.empty : HashMap (String × Nat) Nat) (2 * conf.pattern.size) conf.pattern.toList conf.rule.toList
  |>.snd

def Part2.evaluate (conf : Data) : Nat :=
  let p := Array.foldl (fun s c => s.push c) "" conf.pattern
  let pattern := String.intercalate "?" [p, p, p, p, p]
  let r := conf.rule.toList
  let rule := [r, r, r, r, r].join
  match_sequence (HashMap.empty : HashMap (String × Nat) Nat) (10 * conf.pattern.size) pattern.toList rule
  |>.snd

end Day12

def day12 (ext : Option String) : IO Unit := do
  if let some cs := AoCParser.parse Day12.parser.parser (← dataOf 2023 12 ext) then
    IO.println s!"  Part1: {sum $ cs.map Day12.Part1.evaluate}"
    IO.println s!"  Part2: {sum $ cs.map Day12.Part2.evaluate}"


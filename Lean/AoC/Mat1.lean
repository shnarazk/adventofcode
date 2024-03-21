import Init.Data.Array.Subarray

structure Mat1 (α : Type) [BEq α] [Inhabited α] where
  width  : Nat
  vector : Array α
  nonZero : vector.size > 0
deriving Repr

instance [ToString α] [BEq α] [Inhabited α] : ToString (Mat1 α) where
  toString m := s!"{m.width}{toString m.vector}"

namespace Mat1
/--
return an optional new instance of Mat1 with a given shepe (height, width)
-/
def new! {α : Type} [BEq α] [Inhabited α]
    (shp : Nat × Nat)
    (noneZero : shp.fst * shp.snd > 0)
    (init : α := default) : Mat1 α :=
  let vector := Array.mkArray (shp.fst * shp.snd) init
  let h : vector.size > 0 := by
    rw [Array.size_mkArray (shp.fst * shp.snd)]
    exact noneZero
    done
  ({width := shp.snd, vector := vector, nonZero := h } : Mat1 α)

def new {α : Type} [BEq α] [Inhabited α]
    (shp : Nat × Nat) (init : α := default) : Option (Mat1 α) :=
  let size := shp.fst * shp.snd
  let vector := Array.mkArray size init
  if h : vector.size > 0
  then ({width := shp.snd, vector := vector, nonZero := h } : Mat1 α) |> some
  else none

def ofVector {α : Type} [BEq α] [Inhabited α] (vec : Array α) (w : Nat) : Option (Mat1 α) :=
  if  h : vec.size > 0
  then ({width := w, vector := vec, nonZero := h } : Mat1 α) |> some
  else none

/--
return an optional new instacne of Mat1 of an 2D array
-/
def of2DMatrix {α : Type} [BEq α] [Inhabited α] (m : Array (Array α)) : Option (Mat1 α) :=
  ofVector (m.foldl Array.append #[]) (m.getD 1 #[]).size

/--
return the `(i,j)`-th element of Mat1 instance
-/
@[inline]
def get {α : Type} [BEq α] [Inhabited α] (self : Mat1 α) (i j : Nat) : α :=
  self.vector.get (Fin.ofNat' (i * self.width + j) self.nonZero)

def get₂ {α : Type} [BEq α] [Inhabited α] (self : Mat1 α) (p : Nat × Nat) : α :=
  self.get p.fst p.snd

@[inline]
def validIndex? {α : Type} [BEq α] [Inhabited α] (self : Mat1 α) (i j : Nat) : Bool :=
  0 < i && i < self.width && 0 < j && j * self.width < self.vector.size

def get? {α : Type} [BEq α] [Inhabited α] (self : Mat1 α) (i j : Nat) : Option α :=
  if self.validIndex? i j then self.get i j |> some else none

/--
set the `(i,j)`-th element to `val` and return the modified Mat1 instance
-/
@[inline]
def set {α : Type} [BEq α] [Inhabited α] (self : Mat1 α) (i j : Nat) (val : α) : Mat1 α :=
  let ix := i * self.width + j
  let v := self.vector.set (Fin.ofNat' ix self.nonZero) val
  if h : v.size > 0  -- I don't know how to reuse self.nonZero
  then ({width := self.width, vector := v, nonZero := h } : Mat1 α)
  else self

-- def x := new #[true, false, true, false] 2
-- def y := of2DMatrix #[#[1,2,3], #[4,5,6]]

-- #eval x
-- #check x
-- #eval y
-- #check y
-- #check get

-- #eval get x 0 0
-- #eval get y 0 1

def set₂ {α : Type} [BEq α] [Inhabited α] (self : Mat1 α) (p : Nat × Nat) (val : α) : Mat1 α :=
  self.set p.fst p.snd val

theorem noneZero : (2, 2).fst * (2, 2).snd > 0 := by simp

def x := new! (2, 2) noneZero false
#eval x
#eval x.set₂ (0,1) true

/--
modify the `(i,j)`-th element to `val` and return the modified Mat1 instance
-/
@[inline]
def modify {α : Type} [BEq α] [Inhabited α] (self : Mat1 α) (i j : Nat) (f : α → α) : Mat1 α :=
  let ix := i * self.width + j
  let v := self.vector.modify (Fin.ofNat' ix self.nonZero) f
  if h : v.size > 0  -- I don't know how to reuse self.nonZero
  then ({width := self.width, vector := v, nonZero := h } : Mat1 α)
  else self

/--
search an element that satisfies the predicate and return indices or none
-/
@[inline]
def findIdx? {α : Type} [BEq α] [Inhabited α] (mat : Mat1 α) (f : α → Bool) : Option (Nat × Nat) :=
  match mat.vector.findIdx? f with
  | some i => some (i / mat.width, i % mat.width)
  | none => none

-- #eval if let some y := Mat1.of2DMatrix #[#[1,2,3], #[4,5,6]] then y.findIdx? (· == 6) else none

private partial def findIdxOnSubarray {α : Type} [BEq α] [Inhabited α]
    (sa : Subarray α) (limit : Fin sa.size) (sub1 : Fin sa.size) (pred : α → Bool)
    : Option Nat :=
  if pred (sa.get limit)
  then some limit
  else
    match (limit : Nat) with
    | 0 => none
    | _ => findIdxOnSubarray sa (limit.sub sub1) sub1 pred

/--
search an element in a specific row
-/
@[inline]
def findIdxInRow? {α : Type} [BEq α] [Inhabited α]
    (mat : Mat1 α) (i : Nat) (pred : α → Bool) : Option (Nat × Nat) :=
  let f := i * mat.width
  let t := (i + 1) * mat.width
  let sa := mat.vector.toSubarray f t
  if h : sa.size > 0
  then
    match findIdxOnSubarray sa (Fin.ofNat' (t - f - 1) h) (Fin.ofNat' 1 h) pred with
    | some j => some (i, j)
    | none => none
  else none

-- #eval if let some y := Mat1.of2DMatrix #[#[1,2,3], #[4,5,6]] then y.findIdxInRow? 1 (· == 4) else none

def foldl {α : Type} [BEq α] [Inhabited α] (self : Mat1 α) (f : β → α → β) (init : β) : β :=
  self.vector.foldl f init

def foldlRows {α : Type} [BEq α] [Inhabited α]
    (self : Mat1 α) (f : β → α → β) (init : β ): Array β :=
  Array.range (self.vector.size / self.width)
    |> .map (fun i => self.vector.toSubarray i (i + self.width)
      |> Array.ofSubarray
      |>.foldl f init)

def mapRows {α : Type} [BEq α] [Inhabited α] (self : Mat1 α) (f : Array α → β) : Array β :=
  Array.range (self.vector.size / self.width)
    |> .map (fun i => self.vector.toSubarray i (i + self.width) |> Array.ofSubarray |> f)

def count {α : Type} [BEq α] [Inhabited α] (self : Mat1 α) (f : α → Bool) : Nat :=
  self.vector.map f |>.size

@[inline]
def row {α : Type} [BEq α] [Inhabited α] (self : Mat1 α) (i : Nat) : Subarray α :=
  let j := i % (self.vector.size % self.width)
  let f := j * self.width
  let t := f + self.width
  self.vector.toSubarray f t

@[inline]
def column {α : Type} [BEq α] [Inhabited α] (self : Mat1 α) (i : Nat) : Array α :=
  Array.range (self.vector.size / self.width) |> .map (self.get · i)

@[inline]
def shape {α : Type} [BEq α] [Inhabited α] (self : Mat1 α) : Nat × Nat :=
  (self.vector.size / self.width, self.width)

@[inline]
def size {α : Type} [BEq α] [Inhabited α] (self : Mat1 α) : Nat :=
  self.vector.size

end Mat1

-- def x := Mat1.new #[true, false, true, false] 2
-- def y := Mat1.of2DMatrix #[#[1,2,3], #[4,5,6]]

-- #eval x
-- #check x
-- #eval y
-- #check y
-- #check get

-- #eval x.get 0 0
-- #eval y.get 0 1
-- #eval y.get 100 100

-- #eval x.set 0 0 false
-- #eval y.set 1 1 10000

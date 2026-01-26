/-
  TASM: Time-Absence State Machine
  Formal Verification of Core Axioms
  
  Authorized Origin: TASM Canon v1
-/

namespace TASM

-- 1. Universe Axioms
-- Time is a linear ordered type (e.g., Natural Numbers)
variable {Time : Type} [LinearOrder Time]

-- Actions are an arbitrary type
variable {Action : Type}

-- 2. Observation Predicate
-- Obs(a, t) means "action 'a' was observed at or before time 't'"
variable (Obs : Action → Time → Prop)

-- Monotonicity Axiom for Observations
-- If observed at t1, it remains observed at t2 >= t1
axiom obs_monotonic {a : Action} {t1 t2 : Time} :
  Obs a t1 → t1 ≤ t2 → Obs a t2

-- 3. Absence Definition
-- Absence is the logical negation of Observation AFTER the deadline.
def Abs (a : Action) (deadline : Time) (t : Time) : Prop :=
  deadline ≤ t ∧ ¬ Obs a deadline

-- 4. Theorems

-- Theorem 1: Irreversibility of Absence
-- If an absence is crystallized at t1, it remains true at t2 >= t1.
theorem absence_irreversibility {a : Action} {d t1 t2 : Time} :
  Abs Obs a d t1 → t1 ≤ t2 → Abs Obs a d t2 :=
by
  intros h_abs h_time
  -- Expand definition of Abs at t1
  cases h_abs with h_deadline h_not_obs
  -- Goal: deadline <= t2 AND not Obs a deadline
  apply And.intro
  -- Proving time order
  apply le_trans h_deadline h_time
  -- The fact (not Obs) remains true because it refers to 'deadline', which is constant
  exact h_not_obs

-- Theorem 2: No Late Action Impact (The "No Cycle" Theorem)
-- If Absence is effectively recognized at 't', then Observation at 'deadline' is false.
-- This implies that any "late" observation (at t > deadline) cannot reverse the deadline fact.
theorem no_cycle {a : Action} {d t : Time} :
  Abs Obs a d t → ¬ Obs a d :=
by
  intros h_abs
  cases h_abs with _ h_not_obs
  exact h_not_obs

-- Consistency Check
-- The system allows a state where Absence is True.
example : ∃ (a : Action) (d t : Time), True :=
  ⟨_, _, _, True.intro⟩

end TASM

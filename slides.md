---
marp: true
title: Overloaded Functions
author: Marius Weidner
theme: uncover

paginate: true
math: katex
style: |
    :root {
        font-family: 'JetBrains Mono', serif !important;
        font-variant-ligatures: common-ligatures;
    }
    code {
        font-family: 'JetBrains Mono', serif !important;
        font-variant-ligatures: common-ligatures;
        border-radius: 12px;
    }
    pre {
        font-size: 1rem;
        border-radius: 12px;
    }
    p {
        font-family: 'JetBrains Mono', serif !important;
        font-variant-ligatures: common-ligatures;
    }
    section::after {
        background: none
    }
    .columns {
        display: grid;
        grid-template-columns: repeat(2, minmax(0, 1fr));
        gap: 0.75rem;
    }
    .subtitle {
        font-size: 0.7rem;
        letter-spacing: 0.00000001rem;
    }
    .small {
      font-size: 0.1rem;
    }
    footer {
      color: black;
    }
    
---
<!-- 
Time: 2 min 
Stage: Introduction (5 min)
Desc: -
-->

<!-- _paginate: false -->
<!-- _footer: Marius Weidner ‒ Chair of Programming Languages ‒ Seminar '22 -->

# A Second Look at Overloading

---
<!-- 
Time: 2 min 
Stage: Introduction (5 min)
Desc: Ask who knows Rust / Haskell; explain the one more know about
-->

<style scoped> pre {  font-size: 0.6rem;  }</style>

<div class="columns">
<div> 

```haskell
class Eq α where
  eq :: α -> α -> Bool

instance Eq Nat where
  eq Zero    Zero    = True
  eq (Suc x) (Suc y) = eq x y
  eq _       _       = False



instance Eq α => Eq [α] where
  eq []       []       = True
  eq (x : xs) (y : ys) = eq x y && 
                         eq xs ys
  eq _        _        = False



isEq :: Bool
isEq = eq [Zero] [Zero]
```
<p class="subtitle">Haskell</p>

</div>

<div>

```rust
trait Eq
  fn eq(&self, rhs: &Self) -> Bool

impl Eq for Nat
  fn eq(&self, rhs: &Self) -> Bool
    match (self, rhs)
      (Zero, Zero)     => True,
      (Suc(x), Suc(y)) => x.eq(y),
      (_, _)           => False

impl<A: Eq> Eq for [A]
  fn eq(&self, rhs: &Self) -> Bool 
    match (self, rhs) 
      ([], []) => True,
      ([x, xs@..], [y, ys@..]) 
        => x.eq(y) && xs.eq(ys),
      (_, _)  => False

fn is_eq() -> Bool 
  [Zero].eq(&[Zero])
```
<p class="subtitle">Rust</p>
</div>

</div>


---
<!-- 
Time: 1 min 
Stage: Introduction (5 min)
Desc: Reduce example before to a simpler pseudo code
-->
<style scoped> pre {  font-size: 0.6rem;  }</style>

```haskell
inst eq :: Nat -> Nat -> Bool
  eq Zero    Zero    = True
  eq (Suc x) (Suc y) = eq x y
  eq _       _       = False

inst eq :: (eq :: α -> α -> Bool) => [α] -> [α] -> Bool
  eq []     []       = True
  eq (x:xs) (y:ys)   = eq x y && eq xs ys
  eq _      _        = False

fun isEq :: Bool
  isEq = [Zero] == [Zero]
```

#

<p class="subtitle">Pseudocode</p>

---
<!-- 
Time: 3 min 
Stage: Main (20)
Desc: Explain Hindley Milner syntax, especially let polymorphism & type schemes
-->
<div class="columns">
<div>

$$
\begin{align*}
e :=& \ \ x \\&|
      \ \lambda x. \ e  \\&|
      \ e \ e  \\&|
      \ \textbf{let } x = e \textbf{ in } e  \\

\end{align*}
$$


</div>
<div>

$$
\begin{align*}
\tau :=& \ \ \alpha \\&|
          \ \tau \rightarrow \tau  \\
\sigma :=& \ \ \tau \\&| 
          \ \forall \alpha. \ \sigma

\end{align*}
$$
</div>
</div>

<style scoped> pre {  font-size: 0.7rem;  }</style>


```haskell
 let id   = λx. x             in .. :: ∀α. α -> α 
 let cons = λx. λlst. x : lst in .. :: ∀α. α -> [α] -> [α]         
```
<style scoped> 
  div.error {
    
  }
  div.error > pre {
    font-size: 0.7rem;
    border: 0.06rem;
    background-color: rgb(242, 241, 244); 
    border-color: #B00020;
    border-style: solid;
    border-radius: 12px;
  }
</style>
<div class="error">

```haskell
let evil = λi. id. id i     in .. :: Int -> (∀α. α -> α) -> Int
```

</div>

#

<p class="subtitle">Hindley Milner —— Syntax</p>

---

<!-- 
Time: 2 min 
Stage: Main (20)
Desc: Explain extension with instances + example
-->

<div class="columns">
<div>

$$
\begin{align*}
e :=& \ \ x \\&|
      \ o \quad \tiny{(\text{ if overloaded})} \\&|
      \ k \quad \tiny{(k \in \{\text{unit}, \ 42, \ [e_1,..,e_n], \ .. \})}\\&|
      \ \lambda x. \ e  \\&|
      \ e \ e  \\&|
      \ \textbf{let } x = e \textbf{ in } e  \\
\end{align*}
$$

</div>
<div>

$$
\begin{align*}
p :=& \ \ e \\&| 
       \ \textbf{inst } o : \sigma_T = \sigma \textbf{ in } p \\

\end{align*}
$$

</div>
</div>


<style scoped> pre {  font-size: 0.7rem;  }</style>
```haskell
inst eq : Nat -> Nat -> Bool = λx. λy. x ≐ y in 
inst eq : ∀α. (eq : α -> α -> Bool) => [α] -> [α] -> Bool = 
     | λ[]. λ[]. True
     | λ[x : xs].λ[y : ys]. eq x y && eq xs ys in
 eq [0] [0] 
```

<p class="subtitle">System O —— Syntax</p>

---
<!-- 
Time: 5 min 
Stage: Main (20)
Desc: Explain extension with constraints + type indexed poly types
-->

#

$$
\begin{align*}
\tau :=& \ \ \alpha \\&| 
          \ \tau \rightarrow \tau\\&|
          \ D \ \tau_1 \ ... \ \tau_n \quad \tiny{(D \in \{\text{Unit}, \ \text{Nat}, \ \text{List} \ \tau, ..\}, \ \text{arity}(D) = n)}\\
\pi_\alpha := & \ \ o_1 : \alpha \rightarrow \tau_1, \ ...  \ \  ,o_n : \alpha \rightarrow \tau_n \quad \tiny{(n \in \mathbb{N}, \ o_i \neq o_j)}\\          
\sigma :=& \ \ \tau \\&| 
          \ \forall \alpha. \pi_\alpha \Rightarrow \ \sigma_T \\
\sigma_T :=& \ \ T \ \alpha_1 \ ... \ \alpha_n \rightarrow \tau \quad \tiny{(T \in D \cup \{\rightarrow\}, \ \text{tv}(\tau) \subseteq \{\alpha_1, .., \alpha_n\})} \\&| 
          \  \forall \alpha. \pi_\alpha \Rightarrow \ \sigma_T \quad \tiny{(\text{tv}(\pi_\alpha) \subseteq \text{tv}(\sigma_T) )}
\end{align*}
$$

<p class="subtitle">System O —— Syntax</p>

---
<!-- 
Time: 4 min 
Stage: Main (20)
Desc: Explain relevant typing rules (might insert STLC rules beforehand for intuition)
-->

#



$$
\begin{array}{c c} 
\text{(LET)}
    &
    \displaystyle
    \frac{\Gamma \vdash e_1: \sigma \quad \quad \Gamma, \ x : \sigma \vdash e_2:\tau}
         {\Gamma \vdash \textbf{let} \ x = e_1 \ \textbf{in} \ e_2 : \tau}
    \\\\
    \text{(INST)}
    &
    \displaystyle
    \frac{ \Gamma \vdash e:\sigma_T   \quad \Gamma, \ o : \sigma_T \vdash p : \sigma \quad  \forall (o : \sigma_{T^\prime}) \in \Gamma : T \neq T^\prime}
         {\Gamma \vdash \textbf{inst} \ o : \sigma_T = e \ \textbf{in} \ p : \sigma} 
    \\\\
    \text{(}\boldsymbol\forall\text{I)}
    &
    \displaystyle
    \frac{\Gamma, \ \pi_\alpha\vdash e:\sigma \quad \quad \text{fresh} \ \alpha}
         {\Gamma \vdash e : \forall \alpha. \pi_\alpha \Rightarrow \ \sigma}
    \\\\
    \text{(} \boldsymbol\forall\text{E)}
    &
    \displaystyle
    \frac{\Gamma \vdash e: \forall \alpha. \ \pi_\alpha \Rightarrow \sigma \quad \quad \Gamma \vdash [\tau/\alpha]\pi_\alpha}
         {\Gamma \vdash e:[\tau/\alpha]\sigma}
    
\end{array}\\
$$

#
#

<p class="subtitle">System O —— Typing</p>

---

<!-- 
Time: 3 min 
Stage: Main (20)
Desc: Explain example proof derivation + constraint solving, mention nondeterminism + inference algorithm (similar to algorithm w)
-->


<style scoped> span {  font-size: 0.8rem;  }</style>
$\Gamma = \{ \text{eq} : \text{Nat} \rightarrow \text{Nat} \rightarrow \text{Bool}, \\ \text{eq} : \forall \alpha. \ (\text{eq} : \alpha \rightarrow \alpha \rightarrow \text{Bool}) \Rightarrow [\alpha] \rightarrow [\alpha] \rightarrow \text{Bool} \}$



#

$$
\begin{array}{c}
    \text{eq} : \forall \alpha. \ (\text{eq} : \alpha \rightarrow \alpha \rightarrow \text{Bool}) \qquad\qquad\qquad\qquad\qquad\qquad\qquad\qquad\qquad\qquad \quad \\
    
    \Rightarrow [\alpha] \rightarrow [\alpha] \rightarrow \text{Bool} \in \Gamma \qquad\qquad\qquad\qquad\qquad\qquad\qquad\qquad \qquad \\

    \overline{\Gamma \vdash \text{eq} : \forall \alpha. \ (\text{eq} : \alpha \rightarrow \alpha \rightarrow \text{Bool})} \quad \ \text{Nat} \rightarrow \text{Nat} \rightarrow \text{Bool} \in \Gamma   \qquad \qquad \qquad \quad \\

     \Rightarrow [\alpha] \rightarrow [\alpha] \rightarrow \text{Bool} \quad \quad \quad\overline{\Gamma \vdash \text{Nat} \rightarrow \text{Nat} \rightarrow \text{Bool}} \quad ... \\

    \overline{\qquad \qquad \qquad \quad  \Gamma \vdash \text{eq} : [\text{Nat}] \rightarrow [\text{Nat}] \rightarrow \text{Bool} \qquad \qquad \qquad \quad } \quad  ... \\
    \overline{\ \ \qquad \qquad \qquad \qquad \qquad \Gamma \vdash \text{eq}\ [0] : [\text{Nat}] \rightarrow \text{Bool} \qquad \qquad \qquad \qquad \qquad \ \ } \quad ...\\

    \overline{\ \ \qquad \qquad \qquad \qquad \qquad \qquad \qquad   \Gamma \vdash \text{eq} \ [0] \ [0]  \qquad \qquad \qquad \qquad \qquad \qquad \qquad \ \ }
\end{array}\\
$$
#
#
<p class="subtitle">System O —— Constraint Solving</p>

---

<!-- 
Time: 3 min 
Stage: Main (20)
Desc: Explain example reduction using compositional semantics (very high level, no need to understand the mathematical objects this is translated to)
-->

$$
\begin{align*}
& \ \llbracket \text{inst} \ eq : \mathbb{N} \rightarrow  \mathbb{N} \rightarrow  \mathbb{B} = e_1 \ \text{in} \\ & \ \ \  \text{inst} \ eq : \forall \alpha. \ (\text{eq} : \alpha \rightarrow \alpha \rightarrow \mathbb{B}) \Rightarrow  [\alpha]\rightarrow  [\alpha] \rightarrow  \mathbb{B} = e_2 \ \text{in} \\ & \ \ \ eq \ [0] \ [0] \rrbracket_\emptyset \\
=& \ \llbracket \text{inst} \ eq : \forall \alpha. \ (\text{eq} : \alpha \rightarrow \alpha \rightarrow \mathbb{B}) \Rightarrow  [\alpha] \rightarrow  [\alpha] \rightarrow  \mathbb{B} = .. \ \text{in} \\ & \ \ \ eq \ [0] \ [0] \rrbracket_{\{eq := \lambda x. \ \text{if} \ x \ \text{is} \ \mathbb{N} \ \text{then} \ e_1 \ x \}}\\
=& \ \llbracket eq \ [0] \ [0] \rrbracket_{\{eq := \lambda x. \ \text{if} \ x \ \text{is} \ \text{List} \ \text{then} \ e_2 \ x \ \text{else} \ \lambda x. \ \text{if} \ x \ \text{is} \ \mathbb{N} \ \text{then} \ e_1 \ x \}}\\
=& \ (\lambda x. \ \text{if} \ x \ \text{is} \ [\alpha] \ \text{then} \ e_2 \ x \ \text{else} \ \lambda x. \ \text{if} \ x \ \text{is} \ \mathbb{N} \ \text{then} \ e_1 \ x) \ [0] \ [0] \\
=& \ e_2 \ [0] \ [0]
\end{align*}
$$

<p class="subtitle">System O —— Semantics</p>

---

<!-- 
Time: 2 min 
Stage: Extra (5 min) [skip if no time]
Desc: Explain example translation from system o to Hindley Milner system
-->


<style scoped> pre {  font-size: 0.7rem;  }</style>

```haskell
inst eq : Nat -> Nat -> Bool = λ.. in                 
inst eq : ∀α. (eq : α -> α -> Bool) 
          => [α] -> [α] -> Bool = λ.. in 
 eq [0] [0] 
```
<p class="subtitle">System O</p>

```haskell
let eq₀ :: Nat -> Nat -> Bool
  = λ.. in
let eq₁ :: ∀α. (α -> α -> Bool) -> [α] -> [α] -> Bool
  = λeq₀. λ.. in
 eq₁ eq₀ [0] [0]            
```
<p class="subtitle">Hindley Milner</p>

#


<p class="subtitle">System O —— Translation to Hindley Milner</p>

---

<!-- 
Time: 3 min 
Stage: Extra (5 min) [skip if no time]
Desc: Explain example translation from record calculus with subtyping to system o
-->


<style scoped> pre {  font-size: 0.7rem;  }</style>


```haskell
let max :: ∀β. (gte : β -> β -> Bool) => 
           ∀α. (α <= {key: β}) => α -> α -> α
  = λx. λy. if gte x.key y.key then x else y in
 max {field: "a", key: 1} {field: "b", key: 2}
```
<p class="subtitle">Records + Subtyping</p>

```haskell
inst field : ∀α. ∀β. R₀ α β -> α = λR₀ x y. x in
inst key : ∀α. ∀β. R₀ α β -> β = λR₀ x y. y in
let max :: ∀β. (gte : β -> β -> Bool) => 
           ∀α. (key : α -> β) => α -> α -> α
  = λx. λy. if gte (key x) (key y) then x else y in
 max (R₀ "a" 1) (R₀ "b" 2)
```
<p class="subtitle">System O</p>

# 

<p class="subtitle">System O —— Relationship with Record Typing</p>

---


#### Repository
[github.com/Mari-W/popl](https://github.com/Mari-W/popl)


#### Literatur
- [A Second Look at Overloading](https://dl.acm.org/doi/pdf/10.1145/224164.224195) `1995`<br> Martin Odersky, Philip Wadler, Martin Wehr 
- [A Theory of Type Polymorphism in Programming](https://doi.org/10.1016%2F0022-0000%2878%2990014-4) `1978` <br> Hindley Milner


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

<!-- _paginate: false -->
<!-- _footer: Marius Weidner ‒ Chair of Programming Languages ‒ Seminar '22 -->

# A Second Look at Overloading

---

<style scoped> pre {  font-size: 0.8rem;  }</style>

<!-- <div class="columns">
<div> -->

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
<!-- <p class="subtitle">Haskell</p>

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
      (_ , _)  => False

fn is_eq() -> Bool 
  [Zero].eq(&[Zero])
```
<p class="subtitle">Rust</p>
</div>

</div> -->


<!-- ---

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

<p class="subtitle">Pseudocode</p> -->

---

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

#

<style scoped> pre {  font-size: 0.7rem;  }</style>

```haskell
let id = λx. x : ∀α. α -> α in foo (id "Answer: ") (id 42)
```

#
#

<p class="subtitle">Hindley Milner —— Syntax</p>

---
#

$$
\begin{array}{c c c c} 
    \text{(TAUT)}
    &
    \displaystyle
    \frac{x: \sigma  \in \Gamma}
         {\Gamma \vdash x: \sigma }
    &
    
    \displaystyle
    \frac{\Gamma \vdash e^\prime : \sigma \quad \quad \Gamma,\ x:\sigma \vdash e : \tau}
         {\Gamma \vdash \textbf{let } x = e^\prime \textbf{ in } e : \tau}
    &
    \text{(LET)}
    \\\\
    \text{(}\boldsymbol\rightarrow\text{I}\text{)}
    &
    \displaystyle
    \frac{\Gamma,\ x:\tau \vdash e : \tau^\prime}
         {\Gamma \vdash \lambda x. \ e : \tau \rightarrow \tau^\prime}
    &
    
    \displaystyle
    \frac{\Gamma \vdash e : \tau \rightarrow \tau^\prime \quad\quad \Gamma \vdash e^\prime : \tau}
         {\Gamma \vdash e \ e^\prime : \tau^\prime}
    &
    \text{(}\boldsymbol\rightarrow\text{E}\text{)}
    \\\\
    \text{(}\boldsymbol\forall\text{I}\text{)}
    &
    \displaystyle
    \frac{\Gamma \vdash e:\sigma \quad \quad \text{fresh }\alpha}
         {\Gamma \vdash e : \forall \alpha. \ \sigma}
    &
    \displaystyle
    \frac{\Gamma \vdash e:\sigma \quad \quad \sigma \sqsubseteq \sigma^\prime}
         {\Gamma \vdash e:\sigma^\prime}
    &
    \text{(}\boldsymbol\forall\text{E}\text{)}
\end{array}\\
$$

#
#


<p class="subtitle">Hindley Milner —— Typing</p>

---

###### Let $\Gamma = \{\text{id} : \forall \alpha. \ \alpha \rightarrow \alpha, \ \text{n} : \text{Nat} \}$.

#
#

$$
\begin{array}{c} 
     \text{id} : \forall \alpha. \ \alpha \rightarrow \alpha \in \Gamma \quad \quad
     \forall \alpha. \ \alpha \rightarrow \alpha \sqsubseteq \text{Nat} \rightarrow \text{Nat}
     \quad\quad \ \
     \text{n} :\text{Nat} \in \Gamma\\

     \overline{\quad\quad\quad\quad\quad\quad\quad \Gamma \vdash \text{id} : \text{Nat} \rightarrow \text{Nat}\quad\quad\quad\quad\quad\quad\quad} 
     \quad\quad
     \overline{\ \Gamma \vdash \text{n} :\text{Nat}  }\\

     \overline{\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad \Gamma \vdash \text{id} \ \text{n}                    \quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad }
\end{array}\\
$$
#
#
<p class="subtitle">Hindley Milner —— Instantiation</p>

---

$$
\begin{array}{c}
    x : \alpha \in \{x : \alpha\} \quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad \ \\
    \overline{\ \  x : \alpha \vdash x : \alpha \ \  } \quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\ \ \\
    \quad \quad \quad \overline{\vdash \lambda x. \ x : \alpha \rightarrow \alpha} 
    \quad \quad \quad \quad
    \quad \ \
    \text{id} : \forall \alpha. \ \alpha \rightarrow \alpha \in \{\text{id} : \forall \alpha. \ \alpha  \rightarrow \alpha \} \\

    \overline{\vdash \lambda x. \ x : \forall \alpha. \ \alpha \rightarrow \alpha \quad \text{fresh }\alpha}
    \quad \quad 
    \overline{\ \ \text{id} : \forall \alpha. \ \alpha \rightarrow \alpha \vdash \text{id} : \forall \alpha. \ \alpha \rightarrow \alpha \ \ } \\

    \overline{\quad\quad\quad\quad\quad\quad\quad \quad  \vdash \textbf{let } \text{id} = \lambda x. \ x \textbf{ in } \text{id} : \forall \alpha. \ \alpha \rightarrow \alpha \quad\quad\quad\quad\quad\quad\quad \quad }
\end{array}\\
$$
#
#
<p class="subtitle">Hindley Milner —— Generalization</p>

---

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

<p class="subtitle">System O —— Expressions</p>

---

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

<p class="subtitle">System O —— Types</p>

---
#


$$
\begin{array}{c c} 
    \text{(}\boldsymbol\forall\text{I)}
    &
    \displaystyle
    \frac{\Gamma, \ \pi_\alpha\vdash e:\sigma \quad \quad \text{fresh }\alpha}
         {\Gamma \vdash e : \forall \alpha. \pi_\alpha \Rightarrow \ \sigma}
    \\\\
    \text{(} \boldsymbol\forall\text{E)}
    &
    \displaystyle
    \frac{\Gamma \vdash e: \forall \alpha. \ \pi_\alpha \Rightarrow \sigma \quad \quad \Gamma \vdash [\tau/\alpha]\pi_\alpha}
         {\Gamma \vdash e:[\tau/\alpha]\sigma}
    \\\\
    \text{(SET)}
    &
    \displaystyle
    \frac{\Gamma \vdash x_1:\sigma_1  \quad ...\quad \Gamma \vdash x_n:\sigma_n}
         {\Gamma \vdash x_1:\sigma_1 \quad ...\quad x_n:\sigma_n}
    \\\\
    \text{(INST)}
    &
    \displaystyle
    \frac{ \Gamma \vdash e:\sigma_T   \quad \Gamma, \ o : \sigma_T \vdash p : \sigma \quad  \forall (o : \sigma_{T^\prime}) \in \Gamma : T \neq T^\prime}
         {\Gamma \vdash \textbf{inst } o : \sigma_T = e \textbf{ in } p : \sigma} 
    
\end{array}\\
$$

#
#

<p class="subtitle">System O —— Typing</p>

---


###### Let $\Gamma = \\ \text{eq} : \text{Nat} \rightarrow \text{Nat} \rightarrow \text{Bool}, \\ \ \text{eq} : \forall \alpha. \ (\text{eq} : \alpha \rightarrow \alpha \rightarrow \text{Bool}) \Rightarrow [\alpha] \rightarrow [\alpha] \rightarrow \text{Bool}$



#

$$
\begin{array}{c}
    \text{eq} : \forall \alpha. \ (\text{eq} : \alpha \rightarrow \alpha \rightarrow \text{Bool})\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad \ \ \\
    
    \Rightarrow [\alpha] \rightarrow [\alpha] \rightarrow \text{Bool} \in \Gamma \quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad \quad \ \\

    \overline{\Gamma \vdash \text{eq} : \forall \alpha. \ (\text{eq} : \alpha \rightarrow \alpha \rightarrow \text{Bool})} \quad \ \text{Nat} \rightarrow \text{Nat} \rightarrow \text{Bool} \in \Gamma \quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad \quad\quad\quad\quad  \\

     \Rightarrow [\alpha] \rightarrow [\alpha] \rightarrow \text{Bool} \quad \quad \quad\overline{\Gamma \vdash \text{Nat} \rightarrow \text{Nat} \rightarrow \text{Bool}} \quad\quad\quad\quad \  \  ...\quad\quad\quad\quad\quad\quad\quad\ \\\

    \overline{\quad\quad\quad\quad\quad\quad\quad   \Gamma \vdash \text{eq} : [\text{Nat}] \rightarrow [\text{Nat}] \rightarrow \text{Bool}\quad\quad\quad\quad\quad\quad\quad\quad} \quad \overline{\Gamma \vdash [0] : \text{Nat}} \quad\quad \quad \quad \ \ \ ... \quad \quad \quad   \\

    \overline{ \ \quad\quad\quad\quad\quad\quad\quad\quad\quad \quad\quad\quad\quad   \Gamma \vdash \text{eq}\ [0] : [\text{Nat}] \rightarrow \text{Bool}\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad \ \ \ } \quad  \overline{\Gamma \vdash [0] : \text{Nat}}\\

    \overline{\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad \ \Gamma \vdash \text{eq} \ [0] \ [0]\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad\quad \ }
\end{array}\\
$$
#
#
<p class="subtitle">System O —— Constraint Solving</p>

---



<p class="subtitle">System O —— Compositional Semantics</p>

---



<style scoped> pre {  font-size: 0.7rem;  }</style>

```haskell
inst eq : Nat -> Nat -> Bool = λ.. in 
inst eq : ∀α. (eq : α -> α -> Bool) 
          => [α] -> [α] -> Bool = λ.. in 
 eq [0] [0] 
```
<p class="subtitle">System O</p>

```haskell
let eq_n→n→b = λ.. in
let eq_[α]→[α]→b = λeq_n→n→b. λ.. in
 eq_[α]→[α]→b eq_n→n→b [0] [0]            
```
<p class="subtitle">Hindley Milner</p>

#


<p class="subtitle">System O —— Translation to Hindley Milner</p>



---

#### Slides & Elaboration
[github.com/Mari-W/popl](https://github.com/Mari-W/popl)


#### Literatur
- [A Second Look at Overloading](https://dl.acm.org/doi/pdf/10.1145/224164.224195) `1995`<br> Martin Odersky, Philip Wadler, Martin Wehr 
- [A Theory of Type Polymorphism in Programming](https://doi.org/10.1016%2F0022-0000%2878%2990014-4) `1978` <br> Hindley Milner

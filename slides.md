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
-->

<!-- _paginate: false -->
<!-- _footer: Marius Weidner ‒ Chair of Programming Languages ‒ Seminar '22 -->

<!---
- Paper "A Second Look at Overloading"
- Structure: * An Beispiel Problemstellung erarbeiten
             * Informelle Einführung minimaler Programmiersprache
             * Formelle Erweiterung dieser um überladene Funktionen
             * Beziehung zu dieser zu anderen Programmiersprachen
-->

# A Second Look at Overloading  


---

<!-- 
Time: 1 min 
Stage: Introduction (5 min)
-->

<!--
- Function `eq` mit mehreren Instancen, verschiedenem Verhalten, je nach Typ
- Implicit typ variablen
- Constraint erklären
- In anderen Programmiersprachen: * Type classes in Haskell
                                  * Traits in Rust
                                  * Magic methods in Python
-->
<style scoped> pre {  font-size: 0.6rem;  }</style>

```haskell
inst eq : Nat -> Nat -> Bool
  eq 0     0     = True
  eq (S x) (S y) = eq x y
  eq _     _     = False

inst eq : (eq : α -> α -> Bool) => [α] -> [α] -> Bool
  eq []     []       = True
  eq (x:xs) (y:ys)   = eq x y && eq xs ys
  eq _      _        = False

let isEq = eq [0] [0]
```
#
#

<p class="subtitle">Pseudocode —— Example</p>

---

<!-- 
Time: 3 min 
Stage: Main (20 min)
-->

<div class="columns">
<div>

$$
\begin{align*}
e :=& \ \ x \\&|
      \ \lambda x. \ e  \\&|
      \ e_1 \ e_2  \\&|
      \ \textbf{let } x = e_2 \textbf{ in } e_1  \\

\end{align*}
$$


</div>
<div>

$$
\begin{align*}
\tau :=& \ \ \alpha \\&|
          \ \tau_1 \rightarrow \tau_2  \\
\sigma :=& \ \ \tau \\&| 
          \ \forall \alpha. \ \sigma
\end{align*}
$$
</div>
</div>

<style scoped> pre {  font-size: 0.7rem;  }</style>


```haskell
let cons :: ∀α. α -> [α] -> [α] = λx. λlst. x : lst in ..         
```

 <div class="err">

```haskell
let evil :: (∀α. α -> α) -> Unit = λid. id 42; id "foo"; in ..    
```
```haskell
(λid. .. (id 42) .. (id "foo") ..) (λx. x)                        
```
</div> 

<style scoped>
  div.err > pre {
    font-size: 0.7rem; border: 0.05rem; background-color: rgb(242, 241, 244); border-color: #B00020; border-style: solid; border-radius: 12px;
  }
  div.err {
    padding: 0;
    margin: 0;
  }
  div.err > span {
    margin-left: 0;
  }
</style>
#

<p class="subtitle">Hindley Milner —— Syntax</p>

---

<!-- 
Time: 2 min 
Stage: Main (20 min)
-->

<div class="columns">
<div>

$$
\begin{align*}
e :=& \ \ x \\&|
      \ \lambda x. \ e  \\&|
      \ e_1 \ e_2  \\&|
      \ \textbf{let } x = e_2 \textbf{ in } e_1  \\

p :=& \ \ e \\&| 
       \ \textbf{inst } o : \sigma_T = e \textbf{ in } p \\

\end{align*}
$$

</div>
<div>

$$
\begin{align*}
\tau :=& \ \ \alpha \\&| 
          \ \tau_1 \rightarrow \tau_2\\
\pi_\alpha := & \ \ o_i : \alpha \rightarrow \tau_i \quad \tiny{i\in\mathbb N} \\     
\sigma :=& \ \ \tau \\&| 
          \ \forall \alpha. \pi_\alpha \Rightarrow  \sigma \\
\end{align*}
$$

</div>
</div>


<style scoped> pre {  font-size: 0.7rem;  }</style>
```haskell
inst eq : Nat -> Nat -> Bool = λx. λy. .. in 
inst eq : ∀α. (eq : α -> α -> Bool) => [α] -> [α] -> Bool = .. in
​eq [0] [0] 
```

<p class="subtitle">System O —— Syntax</p>

---

$$
\begin{array}{c c} 
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
         {\Gamma \vdash e : \forall \alpha. \pi_\alpha \Rightarrow  \sigma}
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
Stage: Main (20 min)
-->

<style scoped> span {  font-size: 0.8rem;  }</style>
$\Gamma = \{ \text{eq} : \mathbb{N} \rightarrow \mathbb{N} \rightarrow \mathbb{B}, \\ \text{eq} : \forall \alpha. \ (\text{eq} : \alpha \rightarrow \alpha \rightarrow \mathbb{B}) \Rightarrow [\alpha] \rightarrow [\alpha] \rightarrow \mathbb{B} \}$



#

$$
    \text{eq} : \forall \alpha. \ (\text{eq} : \alpha \rightarrow \alpha \rightarrow \mathbb{B}) \qquad\qquad\qquad\qquad\qquad\qquad\qquad\qquad\qquad\qquad\qquad\quad\quad\ \ \ \\
    
    \Rightarrow [\alpha] \rightarrow [\alpha] \rightarrow \mathbb{B} \in \Gamma \qquad\qquad\qquad\qquad\qquad\qquad\qquad\qquad\qquad \qquad \\

    \overline{\  \Gamma \vdash \text{eq} : \forall \alpha. \ (\text{eq} : \alpha \rightarrow \alpha \rightarrow \mathbb{B}) \ } \qquad \text{eq} : \mathbb{N} \rightarrow \mathbb{N} \rightarrow \mathbb{B} \in \Gamma \qquad\qquad\qquad\qquad\qquad\qquad\quad   \\

    \Rightarrow [\alpha] \rightarrow [\alpha] \rightarrow \mathbb{B} \quad \quad \quad 
    \overline{\Gamma \vdash \text{eq} : \mathbb{N} \rightarrow \mathbb{N} \rightarrow \mathbb{B}} \qquad \quad \ ... \qquad\quad \\

    \overline{\qquad\qquad\qquad\quad \quad \Gamma \vdash \text{eq} : [\mathbb{N}] \rightarrow [\mathbb{N}] \rightarrow \mathbb{B}\qquad\qquad\qquad\quad\quad } \quad \overline{\Gamma \vdash [0] : [\mathbb{N}]} \qquad\quad \ ... \quad \ \ \ \\\

    \overline{\qquad\qquad\qquad\qquad\qquad\qquad \ \Gamma \vdash \text{eq}\ [0] : [\mathbb{N}] \rightarrow \mathbb{B}\qquad\qquad\qquad\qquad\qquad\qquad} \quad \overline{\Gamma \vdash [0] : [\mathbb{N}]} \\

    \overline{\qquad\qquad\qquad\qquad\qquad\qquad\qquad\qquad \Gamma \vdash \text{eq} \ [0] \ [0] : \mathbb{B} \qquad\qquad\qquad\qquad\qquad\qquad\qquad\qquad \ }
$$
#
#
<p class="subtitle">System O —— Constraint Solving</p>

---

<!-- 
Time: 3 min 
Stage: Main (20 min)
-->

$$
\begin{align*}
& \ \llbracket \text{inst} \ eq : \mathbb{N} \rightarrow  \mathbb{N} \rightarrow  \mathbb{B} = e_1 \ \text{in} \\ & \ \ \  \text{inst} \ eq : \forall \alpha. \ (\text{eq} : \alpha \rightarrow \alpha \rightarrow \mathbb{B}) \Rightarrow  [\alpha]\rightarrow  [\alpha] \rightarrow  \mathbb{B} = e_2 \ \text{in} \\ & \ \ \ eq \ [0] \ [0] \rrbracket_\emptyset \\
=& \ \llbracket \text{inst} \ eq : \forall \alpha. \ (\text{eq} : \alpha \rightarrow \alpha \rightarrow \mathbb{B}) \Rightarrow  [\alpha] \rightarrow  [\alpha] \rightarrow  \mathbb{B} = .. \ \text{in} \\ & \ \ \ eq \ [0] \ [0] \rrbracket_{\{eq := \lambda x. \ \text{if} \ x \ \text{is} \ \mathbb{N} \ \text{then} \ \llbracket e_1 \rrbracket \ x \}}\\
=& \ \llbracket eq \ [0] \ [0] \rrbracket_{\{eq := \lambda x. \ \text{if} \ x \ \text{is} \ \text{List} \ \text{then} \ \llbracket e_2 \rrbracket \ x \ \text{else} \ \text{if} \ x \ \text{is} \ \mathbb{N} \ \text{then} \ \llbracket e_1 \rrbracket \ x \}}\\
=& \ (\lambda x. \ \text{if} \ x \ \text{is} \ \text{List} \ \text{then} \ \llbracket e_2 \rrbracket \ x \ \text{else} \ \text{if} \ x \ \text{is} \ \mathbb{N} \ \text{then} \ \llbracket e_1 \rrbracket \ x) \ [0] \ [0] \\
=& \ \llbracket e_2 \rrbracket \ [0] \ [0] \\
=& \ \text{true}
\end{align*}
$$

#
<p class="subtitle">System O —— Semantics</p>

---

<!-- 
Time: 2 min 
Stage: Extra (5 min) [skip if no time]
-->


<style scoped> pre {  font-size: 0.7rem;  }</style>

```haskell
inst eq : Nat -> Nat -> Bool 
  = λ.. in                 
inst eq : ∀α. (eq : α -> α -> Bool) => [α] -> [α] -> Bool 
  = λ.. in
​eq [0] [0] 
```
<p class="subtitle">System O</p>

```haskell
let eq₀ :: Nat -> Nat -> Bool
  = λ.. in
let eq₁ :: ∀α. (α -> α -> Bool) -> [α] -> [α] -> Bool     
  = λeq₀. λ.. in
​eq₁ eq₀ [0] [0]            
```
<p class="subtitle">Hindley Milner</p>

#


<p class="subtitle">System O —— Translation to Hindley Milner</p>

---

<!-- 
Time: 3 min 
Stage: Extra (5 min) [skip if no time]
-->


<style scoped> pre {  font-size: 0.7rem;  }</style>


```haskell
let max :: ∀β. (gte : β -> β -> Bool) => 
           ∀α. (α <= {key: β}) => α -> α -> α      
  = λx. λy. if gte x.key y.key then x else y in
​​max {field: "a", key: 1} {field: "b", key: 2}
```
<p class="subtitle">Records + Subtyping</p>

```haskell
inst field : ∀α. ∀β. R₀ α β -> α = λR₀ x y. x in
inst key : ∀α. ∀β. R₀ α β -> β = λR₀ x y. y in
let max :: ∀β. (gte : β -> β -> Bool) => 
           ∀α. (key : α -> β) => α -> α -> α
  = λx. λy. if gte (key x) (key y) then x else y in
​max (R₀ "a" 1) (R₀ "b" 2)
```
<p class="subtitle">System O</p>

# 

<p class="subtitle">System O —— Relationship with Record Typing</p>

---
<style scoped> li {  font-size: 0.7rem;  }</style>

##### Repository
[github.com/Mari-W/popl](https://github.com/Mari-W/popl)
#
#

##### References
- [A Second Look at Overloading](https://dl.acm.org/doi/pdf/10.1145/224164.224195) `1995`<br> Martin Odersky, Philip Wadler, Martin Wehr 
- [A Theory of Type Polymorphism in Programming](https://doi.org/10.1016%2F0022-0000%2878%2990014-4) `1978` <br> Hindley Milner


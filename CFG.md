S -> A

A -> <Type><VName>(B){D} 

B -> <Type><VName>B | , | ε

C -> if(<Ex>){C} | for(<Ex>;<Ex>;<Ex>){C} | while(<Ex>){C} | D

D -> const<Type><VName>=<Ex>; | <Type><VName>=<Ex>; | <Type><VName>; | <VName>=<Ex>;

<Type> -> int | double

<VName> -> Σ∗

<Ex> -> <Tm>+<Ex> | <Tm>-<Ex> | <Tm><BEx><Ex> | <Tm>

<BEx> -> && | || | > | < | >= | <= | <> | ==

<Tm> -> <Fr>*<Tm> | <Fr>/<Tm> | <Fr>%<Tm> | <Fr>*<Tm> | <Fr>

<Fr> -> (<Ex>) | ZZ | RR
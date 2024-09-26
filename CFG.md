S -> A

A -> if(<Ex>){D} | for(<Ex>;<Ex>;<Ex>){D} | while(<Ex>){D} | <Type><VName>(B){D}

B -> <Type><VName>B | , | ε

C -> const<Type><VName>=<Ex>; | <Type><VName>=<Ex>; | <Type><VName>; | <VName>=<Ex>;

D -> A | D

<Type> -> int | double

<VName> -> Σ∗

<Ex> -> <Tm>+<Ex> | <Tm>-<Ex> | <Tm><BEx><Ex> | <Tm>

<BEx> -> && | || | > | < | >= | <= | <> | ==

<Tm> -> <Fr>*<Tm> | <Fr>/<Tm> | <Fr>%<Tm> | <Fr>*<Tm> | <Fr>

<Fr> -> (<Ex>) | ZZ | RR
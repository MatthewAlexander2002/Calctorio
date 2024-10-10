First(Prog)= First(FuncList) = First(FuncDecl) = First(Decl) = First(type) = { int | double }
First(FuncList) = First(FuncDecl) = First(Decl) = First(type) = { int | double }
First(FuncList) = ε
First(FuncDecl) = First(Decl) = First(type) = { int | double }
First(ListOfParams) = First(NonEmptyListOfParams) = { , | ε | int | double }
First(ListOfParams) = ε
First(NonEmptyListOfParams) = First(Decl) = First(type) = { int | double }
First(NonEmptyListOfParams) = First(NonEmptyListOfParamsContinue) = { , | ε }
First(NonEmptyListOfParamsContinue) = ,
First(NonEmptyListOfParamsContinue) = ε
First(StatementList) = First(Statement) = {if | while | for | Σ∗ | const | int | double | break | continue | return | print | ε}
First(StatementList) ε
First(Statement) = if
First(Statement) = while
First(Statement) = for
First(Statement) = First(assignment) = First(VName) = Σ∗
First(Statement) = First(VarDecl) = { const | int | double }
First(Statement) = break
First(Statement) = continue
First(Statement) = return
First(Statement) = print
First(Statement) = ε
First(forLoopFirstBit) = First(VarDecl) = { const | int | double }
First(forLoopFirstBit) = First(assignment) = First(VName) = Σ∗
First(forLoopFirstBit) = ε
First(forLoopLastBit) = First(assignment) = First(VName) = Σ∗
First(forLoopLastBit) = ε
First(returnTail) = First(Number) = {RR | ZZ}
First(returnTail) = First(VName) = Σ∗
First(Text) = First(TextElement) = {RR | ZZ | Σ∗} 
First(Text) = ε
First(TextElement) = Σ∗
First(TextElement) = First(Number) = {RR | ZZ}
First(TextElement) = First(VName) = Σ∗
First(TextTail) = First(TextElement) = {RR | ZZ | Σ∗} 
First(TextTail) = ε
First(assignment) = First(VName) = Σ∗
First(VarDecl) = const
First(VarDecl) = First(Decl) = First(type) = { int | double }
First(VarDecl) = First(Decl) = First(type) = { int | double }
First(Decl) = First(type) = { int | double }
First(Ex) = First(BoolEx) = First(RelEx) = First(ArithEx) = {RR | ZZ | Σ∗ | ( | toINT | toDOUBLE } 
First(Ex) = First(ArithEx) = {RR | ZZ | Σ∗ | ( | toINT | toDOUBLE }
First(BoolEx) = First(RelEx) = First(ArithEx) = {RR | ZZ | Σ∗ | ( | toINT | toDOUBLE }
First(BoolEx') = First(BoolOp) = { && | || }
First(BoolEx') ε
First(BoolOp) = &&
First(BoolOp) = || 
First(RelEx) = First(ArithEx) = {RR | ZZ | Σ∗ | ( | toINT | toDOUBLE }
First(RelEx') = First(RelOp) = { == | > | < | >= | <= | <> }
First(RelEx') = ε
First(RelOp) = ==
First(RelOp) = >
First(RelOp) = <
First(RelOp) = >=
First(RelOp) = <=
First(RelOp) = <>
First(ArithEx) = First(ArithVal) = {RR | ZZ | Σ∗}
First(ArithEx) = (
First(ArithEx) = toINT
First(ArithEx) = toDOUBLE
First(ArithEx') = First(ArithOp) = { + | - }
First(ArithEx') = ε
First(ArithOp) = +
First(ArithOp) = -
First(ArithOp) = First(ArithOp') = { * | / | %}
First(ArithOp') = *
First(ArithOp') = / 
First(ArithOp') = %
First(ArithVal) = First(fnCall) = First(VName) = Σ∗
First(ArithVal) = First(Number) = {RR | ZZ}
First(ArithVal) = First(VName) = Σ∗
First(fnCall) = First(VName) = Σ∗
First(argList) = First(Ex) = First(BoolEx) = First(RelEx) = First(ArithEx) = {RR | ZZ | Σ∗ | ( | toINT | toDOUBLE }, First(Ex) = First(ArithEx) = {RR | ZZ | Σ∗ | ( | toINT | toDOUBLE }
First(argList) = ε
First(argListTail) = ,
First(argListTail) = ε
First(type) = int
First(type) = double
First(VName) = Σ∗
First(Number) = ZZ
First(Number) = RR
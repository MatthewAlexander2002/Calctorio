First(Prog) -> First(FuncList)
First(FuncList) -> First(FuncDecl) U {ε}
First(FuncDecl) -> First(Decl)
First(ListOfParams) -> First(NonEmptyListOfParams) U {ε}
First(NonEmptyListOfParams) -> First(Decl) U First(NonEmptyListOfParamsContinue)
First(NonEmptyListOfParamsContinue) -> {, | ε}
First(StatementList) -> First(Statement) U {ε}
First(Statement) -> First(assignment) U First(VarDecl) U { if | while | for | break | continue | return | print | ε } //ChatGPT: First(assignment) U First(VarDecl) U { if, while, for, break, continue, return, print } no ε
First(forLoopFirstBit) -> First(VarDecl) U First(assignment) U {ε}
First(forLoopLastBit) -> First(assignment) U {ε}
First(returnTail) -> First(Number) U First(VName)
First(Text) -> First(TextElement) U {ε}
First(TextElement) -> <String> U First(Number) U First(VName)
First(TextTail) -> + First(TextElement) U {ε}
First(assignment) -> First(VName)
First(VarDecl) -> {const} U First(Decl)
First(VarDecl') -> {= | ε}
First(Decl) -> First(type)
First(Ex) -> First(BoolEx)
First(BoolEx) -> First(RelEx)
First(BoolEx') -> First(BoolOp) U {ε}
First(BoolOp) -> {&& | ||}
First(RelEx) -> First(ArithEx)
First(RelEx') -> First(RelOp) U {ε}
First(RelOp) -> {== | > | < | >= | <= | <>}
First(ArithEx) -> First(ArithVal) U {( | toINT | toDOUBLE} 
First(ArithEx') -> First(ArithOp)  U {ε}
First(ArithOp) -> First(ArithOp') U {+ | -} 
First(ArithOp') -> {* | / | %}
First(ArithVal) -> First(fnCall) U First(Number) U First(VName)
First(fnCall) -> First(VName)
First(argList) -> First(Ex) U {ε}
First(argListTail) -> {, | ε}
First(type) -> {int | double}
First(VName) -> {Σ∗}
First(Number) -> {ZZ | RR}
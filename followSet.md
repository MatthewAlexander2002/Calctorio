Follow(Prog) -> $
Follow(FuncList) -> Follow(Prog)
Follow(FuncDecl) -> First(FuncList) U Follow(FuncList)
Follow(ListOfParams) -> {)}
Follow(NonEmptyListOfParams) -> Follow(ListOfParams)
Follow(NonEmptyListOfParamsContinue) -> Follow(NonEmptyListOfParams)
Follow(StatementList) -> {\}}
Follow(Statement) -> First(StatementList) U Follow(StatementList)
Follow(forLoopFirstBit) -> {;}
Follow(forLoopLastBit) -> {)}
Follow(returnTail) -> {;}
Follow(Text) -> {)}
Follow(TextElement) -> Follow(TextTail)
Follow(TextTail) -> Follow(Text)
Follow(assignment) -> Follow(forLoopFirstBit) U Follow(forLoopLastBit) U Follow(Statement)
Follow(VarDecl) -> Follow(forLoopFirstBit) U Follow(Statement) 
Follow(Decl) -> Follow(NonEmptyListOfParamsContinue) U Follow(NonEmptyListOfParams) U {; | = | (}
Follow(Ex) -> First(argListTail) U {;}
Follow(BoolEx) -> Follow(Ex)
Follow(BoolEx') -> Follow(BoolEx)
Follow(BoolOp) -> First(BoolEx)
Follow(RelEx) -> First(BoolEx')
Follow(RelEx') -> Follow(RelEx)
Follow(RelOp) -> First(ArithEx)
Follow(ArithEx) -> First(ArithEx') U Follow(ArithEx') U {)}
Follow(ArithEx') -> Follow(ArithEx)
Follow(ArithOp) -> First(ArithEx)
Follow(ArithOp') -> Follow(ArithOp)
Follow(ArithVal) -> First(ArithEx')
Follow(fnCall) -> <VName>(<argList>)
Follow(argList) -> <Ex><argListTail>
Follow(argList) -> ε
Follow(argListTail) -> ,<Ex><argListTail>
Follow(argListTail) -> ε
Follow(type) -> int
Follow(type) -> double
Follow(VName) -> Σ∗
Follow(Number) -> ZZ
Follow(Number) -> RR
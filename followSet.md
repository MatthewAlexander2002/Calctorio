Follow(Prog) -> $
Follow(FuncList) -> Follow(Prog)
Follow(FuncDecl) -> First(FuncList) U Follow(FuncList)
Follow(ListOfParams) -> {)}
Follow(NonEmptyListOfParams) -> Follow(ListOfParams)
Follow(NonEmptyListOfParamsContinue) -> Follow(NonEmptyListOfParams)
Follow(StatementList) -> {}}
Follow(Statement) -> First(StatementList) U Follow(StatementList)
Follow(forLoopFirstBit) -> {;} // First(BoolEx)
Follow(forLoopLastBit) -> {)}
Follow(returnTail) -> {;}
Follow(Text) -> {)}
Follow(TextElement) -> Follow(TextTail) U Follow(Text)
Follow(TextTail) -> Follow(Text)
Follow(assignment) -> Follow(forLoopFirstBit) U Follow(forLoopLastBit) U Follow(Statement)
Follow(VarDecl) -> Follow(forLoopFirstBit) U Follow(Statement)
Follow(VarDecl') -> {;}
Follow(Decl) -> Follow(NonEmptyListOfParamsContinue) U Follow(NonEmptyListOfParams) U {; | = | (}
Follow(Ex) -> First(argListTail) U {;}
Follow(BoolEx) -> Follow(Ex)
Follow(BoolEx') -> Follow(BoolEx)
Follow(BoolOp) -> First(BoolEx)
Follow(RelEx) -> First(BoolEx') U Follow(BoolEx)
Follow(RelEx') -> Follow(RelEx)
Follow(RelOp) -> First(ArithEx)
Follow(ArithEx) -> First(ArithEx') U Follow(ArithEx') U Follow(RelEx) U {)}
Follow(ArithEx') -> Follow(ArithEx)
Follow(ArithOp) -> First(ArithEx)
Follow(ArithOp') -> Follow(ArithOp)
Follow(ArithVal) -> First(ArithEx') U Follow(ArithEx)
Follow(fnCall) -> Follow(ArithVal)
Follow(argList) -> {)}
Follow(argListTail) -> Follow(argList)
Follow(type) -> First(VName)
Follow(VName) -> Follow(ArithVal) U Follow(Decl) U Follow(TextElement) U Follow(returnTail) {( | =}
Follow(Number) -> Follow(ArithVal) U Follow(TextElement) U Follow(returnTail)
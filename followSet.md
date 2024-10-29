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
Follow(Ex) -> <BoolEx> 
Follow(Ex) -> <ArithEx> 
Follow(BoolEx) -> <RelEx><BoolEx'> 
Follow(BoolEx') -> <BoolOp><BoolEx>
Follow(BoolEx') ε
Follow(BoolOp) -> &&
Follow(BoolOp) -> || 
Follow(RelEx) -> <ArithEx><RelEx'>
Follow(RelEx') -> <RelOp><ArithEx>
Follow(RelEx') -> ε
Follow(RelOp) -> ==
Follow(RelOp) -> >
Follow(RelOp) -> <
Follow(RelOp) -> >=
Follow(RelOp) -> <=
Follow(RelOp) -> <>
Follow(ArithEx) -> <ArithVal><ArithEx'>
Follow(ArithEx) -> (<ArithEx>)
Follow(ArithEx) -> toINT(<ArithEx>);
Follow(ArithEx) -> toDOUBLE(<ArithEx>);
Follow(ArithEx') -> <ArithOp><ArithEx>
Follow(ArithEx') -> ε
Follow(ArithOp) -> +
Follow(ArithOp) -> -
Follow(ArithOp) -> <ArithOp'>
Follow(ArithOp') -> *
Follow(ArithOp') -> / 
Follow(ArithOp') -> %
Follow(ArithVal) -> <fnCall>
Follow(ArithVal) -> <Number>
Follow(ArithVal) -> <VName>
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
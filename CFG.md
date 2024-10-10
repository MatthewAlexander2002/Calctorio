<Prog> -> <FuncList>
<FuncList> -> <FuncDecl><FuncList> 
<FuncList> -> ε
<FuncDecl> -> <Decl>(<ListOfParams>){<StatementList>}
<ListOfParams> -> <NonEmptyListOfParams> 
<ListOfParams> -> ε
<NonEmptyListOfParams> -> <Decl> 
<NonEmptyListOfParams> -> <NonEmptyListOfParamsContinue>
<NonEmptyListOfParamsContinue> -> ,<Decl> 
<NonEmptyListOfParamsContinue> -> ε
<StatementList> -> <Statement><StatementList> 
<StatementList> ε
<Statement> -> if(<BoolEx>){<StatementList>}
<Statement> -> while(<BoolEx>){<StatementList>}
<Statement> -> for(<forLoopFirstBit>; <BoolEx>; <forLoopLastBit>){<StatementList>}
<Statement> -> <assignment>
<Statement> -> <VarDecl>
<Statement> -> break;
<Statement> -> continue;
<Statement> -> <return>
<Statement> -> <print>
<Statement> -> ε
<forLoopFirstBit> -> <VarDecl> 
<forLoopFirstBit> -> <assignment>
<forLoopFirstBit> -> ε
<forLoopLastBit> -> <assignment> 
<forLoopLastBit> -> ε
<return> -> return<returnTail>;
<returnTail> -> <number>
<returnTail> -> <VName>
<print> -> print(<Text>);
<Text> -> <TextElement><TextTail>
<Text> -> ε
<TextElement> -> <String>
<TextElement> -> <number>
<TextElement> -> <VName>
<TextTail> -> + <TextElement><TextTail>
<TextTail> -> ε
<assignment> -> <VName>=<Ex>;
<VarDecl> -> const<Decl>=<Ex>;
<VarDecl> -> <Decl>=<Ex>; 
<VarDecl> -> <Decl>;
<Decl> -> <Type><VName>
<Ex> -> <BoolEx> 
<Ex> -> <ArithEx> 
<BoolEx> -> <RelEx><BoolEx'> 
<BoolEx'> -> <BoolOp><BoolEx>
<BoolEx'> ε
<BoolOp> -> &&
<BoolOp> -> || 
<RelEx> -> <ArithEx><RelEx'>
<RelEx'> -> <RelOp><ArithEx>
<RelEx'> -> ε
<RelOp> -> ==
<RelOp> -> >
<RelOp> -> <
<RelOp> -> >=
<RelOp> -> <=
<RelOp> -> <>
<ArithEx> -> <ArithVal><ArithEx'>
<ArithEx> -> (<ArithEx>)
<ArithEx> -> toINT(<ArithEx>);
<ArithEx> -> toDOUBLE(<ArithEx>);
<ArithEx'> -> <ArithOp><ArithEx>
<ArithEx'> -> ε
<ArithOp> -> +
<ArithOp> -> -
<ArithOp> -> <ArithOp'>
<ArithOp'> -> *
<ArithOp'> -> / 
<ArithOp'> -> %
<ArithVal> -> <fnCall>
<ArithVal> -> <Number>
<ArithVal> -> <VName>
<fnCall> -> <VName>(<argList>)
<argList> -> <Ex><argListTail>
<argList> -> ε
<argListTail> -> ,<Ex><argListTail>
<argListTail> -> ε
<VName> -> Σ∗
<type> -> int
<type> -> double
<Number> -> ZZ
<Number> -> RR

first and follow sets -> table

number my rules & and expand ors to its own lines
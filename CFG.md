<Prog> -> <FuncList>

<FuncList> -> <FuncDecl><FuncList> | ε

<FuncDecl> -> <Decl>(<ListOfParams>){<StatementList>}

<ListOfParams> -> <NonEmptyListOfParams> | ε

<NonEmptyListOfParams> -> <Decl> | <NonEmptyListOfParamsContinue>

<NonEmptyListOfParamsContinue> -> ,<Decl> | ε

<StatementList> -> <Statement><StatementList> | ε

<Statement> -> if(<BoolEx>){<StatementList>} | while(<BoolEx>){<StatementList>} | for(<forLoopFirstBit>; <BoolEx>; <forLoopLastBit>){<StatementList>} | <assignment> | <VarDecl> | break; | continue; | <return> | <print> | ε

<forLoopFirstBit> -> <VarDecl> | <assignment> | ε

<forLoopLastBit> -> <assignment> | ε

<return> -> return<returnTail>;

<returnTail> -> <number> | <VName>

<print> -> print(<Text>); 

<Text> -> <TextElement><TextTail> | ε

<TextElement> -> <String> | <number> | <VName>

<TextTail> -> + <TextElement><TextTail> | ε

<assignment> -> <VName>=<Ex>;

<VarDecl> -> const<Decl>=<Ex>; | <Decl>=<Ex>; | <Decl>;

<Decl> -> <Type><VName>

<Ex> -> <BoolEx> | <ArithEx> 

<BoolEx> -> <RelEx><BoolEx'> 

<BoolEx'> -> <BoolOp><BoolEx> | ε

<BoolOp> -> && | || 

<RelEx> -> <ArithEx><RelEx'>

<RelEx'> -> <RelOp><ArithEx> | ε

<RelOp> -> == | > | < | >= | <= | <>

<ArithEx> -> <ArithVal><ArithEx'> | (<ArithEx>) | toINT(<ArithEx>); | toDOUBLE(<ArithEx>);

<ArithEx'> -> <ArithOp><ArithEx> | ε

<ArithOp> -> + | - | <ArithOp'>

<ArithOp'> -> * | / | %

<ArithVal> -> <fnCall> | <Number> | <VName>

<fnCall> -> <VName>(<argList>)

<argList> -> <Ex><argListTail> | ε

<argListTail> -> ,<Ex><argListTail> | ε

<VName> -> Σ∗

<type> -> int | double

<Number> -> ZZ | RR

first and follow sets -> table

number my rules & and expand ors to its own lines
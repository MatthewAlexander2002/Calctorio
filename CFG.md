<Prog> -> <FuncList>

<FuncList> -> <FuncDecl><FuncList> | ε

<FuncDecl> -> <Decl>(<ListOfParams>){<StatementList>}

<ListOfParams> -> <NonEmptyListOfParams> | ε

<NonEmptyListOfParams> -> <Decl> | <NonEmptyListOfParamsContinue>

<NonEmptyListOfParamsContinue> -> ,<Decl> | ε

<StatementList> -> <Statement><StatementList> | ε

<Statement> -> <ifStatement> | <whileLoop> | <forLoop> | <assignment> | <VarDecl> | break; | continue; | <return> | <print> | <comment> | ε

<ifStatement> -> if(<BoolEx>){<StatementList>} 

<whileLoop> -> while(<BoolEx>){<StatementList>}

<forLoop> -> for(<Ex>; <BoolEx>; <Ex>){<StatementList>}

<return> -> return<returnTail>;

<returnTail> -> <number> | <VName>

<print> -> print(<Text>);

<comment> -> /*<Text>*/ | //<Text'>

<Text> -> <WhiteSpace><VName><Text><WhiteSpace> | ε

<Text'> -> <WhiteSpace'><VName><Text'><WhiteSpace'><NewLine> | ε

<WhiteSpace> -> <Space><WhiteSpace> | <NewLine><WhiteSpace> | ε

<WhiteSpace'> -> <Space><WhiteSpace> | ε

<assignment> -> <VName>=<Ex>;

<VarDecl> -> const<Decl>=<Ex>; | <Decl>=<Ex>; | <Decl>;

<Decl> -> <Type><VName>

<Ex> -> <BoolEx> | <ArithEx> 

<BoolEx> -> <RelEx><BoolEx'> 

<BoolEx'> -> <BoolOp><BoolEx> | ε

<BoolOp> -> && | || 

<RelEx> -> <ArithEx><RelOp><ArithEx>

<RelOp> -> == | > | < | >= | <= | <>

<ArithEx> -> <ArithVal><ArithEx'> | (<ArithEx>) | <toINT> | <toDOUBLE>

<ArithEx'> -> <ArithOp><ArithEx> | ε

<ArithOp> -> + | - | <ArithOp'>

<ArithOp'> -> * | / | %

<ArithVal> -> <fnCall> | <Number> | <VName>

<fnCall> -> <VName>(<argList>)

<argList> -> <Ex><argListTail> | ε

<argListTail> -> ,<Ex><argListTail> | ε

<VName> -> Σ∗

<toINT> -> toINT(<ArithEx>);

<toDOUBLE> -> toDOUBLE(<ArithEx>);

<type> -> int | double

<Number> -> ZZ | RR

first and follow sets -> table
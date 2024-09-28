<Prog> -> <FuncList>

<FuncList> -> <FuncDecl><FuncList> | ε

<FuncDecl> -> <Decl>(<ListOfParams>){<StatementList>}

<ListOfParams> -> <NonEmptyListOfParams> | ε

<NonEmptyListOfParams> -> <Decl> | <NonEmptyListOfParamsContinue>

<NonEmptyListOfParamsContinue> -> ,<Decl> | ε

<StatementList> -> <Statement><StatementList> | ε

<Statement> -> <ifStatement> | <whileLoop> | <forLoop> | <assignment> | <VarDecl> 

<ifStatement> -> if(<BoolEx>){<StatementList>} 

<whileLoop> -> while(<BoolEx>){<StatementList>}

<forLoop> -> 

<assignment> ->

<VarDecl> ->

<Ex> -> <BoolEx> | <ArithEx>

<BoolEx> -> <RelEx><BoolEx'> 

<BoolEx'> -> <BoolOp><BoolEx> | ε

<BoolOp> -> && | || //might need more

<RelEx> -> <ArithEx><RelOp><ArithEx>

<RelOp> -> == | > | < | >= | <= | <>

<ArithEx> -> <ArithVal><ArithEx'> | (<ArithEx>) //need to make sure the brackets exicute first they are quite high up the stack or is this a rule that i done at a later set  // maybe | <ArithVal> 

<ArithEx'> -> <ArithOp><ArithEx> | ε

<ArithOp> -> + | - | <ArithOp'>

<ArithOp'> -> * | / | %

<ArithVal> -> <fnCall> | <Number> | <VName>

<fnCall> -> <VName>(<argList>)

<Number> -> //need to figure out how to int or double

<argList> -> <ex><argListTail> | ε

<argListTail> -> ,<Ex><argListTail> | ε




C -> if(<Ex>){C} | for(<Ex>;<BEx>;<Ex>){C} | while(<BEx>){C} | D

D -> const<Decl> =<Ex>; | <Decl> =<Ex>; | <Decl>; | <VName>=<Ex>;

<Decl> -> <Type><VName>

<Type> -> int | double

<VName> -> Σ∗

<Ex> -> <Tm>+<Ex> | <Tm>-<Ex> | <Tm><BEx><Ex> | <Tm>

<BEx> -> && | || | > | < | >= | <= | <> | ==

<Tm> -> <Fr>*<Tm> | <Fr>/<Tm> | <Fr>%<Tm> | <Fr>

<Fr> -> (<Ex>) | ZZ | RR

first and follow sets -> table
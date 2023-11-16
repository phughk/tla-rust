/// This is the uppermost struct that is parsed from a file
pub struct TlaSpec {
    pub module_block: ModuleBlock,
    pub extends_block: ExtendsBlock,
    pub define_block: DefineBlock,
    pub variable_block: VariableBlock,
    pub invariant_block: InvariantBlock,
    pub theorem_block: TheoremBlock,
}

pub struct ModuleBlock {

}

pub struct ExtendsBlock {
    pub modules: Vec<ExtendModuleDeclaration>,
}

pub struct ExtendModuleDeclaration {
    pub module_name: String,
    pub operators: Vec<OperatorDefinition>,
}

pub struct DefineBlock {

}

pub struct InvariantBlock {

}

pub struct VariableBlock {

}

pub struct TheoremBlock {

}

pub struct OperatorDefinition {

}

/// Syntax: Op(arg1, arg2) == Expr
pub struct Operator {
    pub definition: OperatorDefinition,
}
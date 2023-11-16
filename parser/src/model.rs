/// This is the uppermost struct that is parsed from a file
pub struct TlaSpec {
    pub module_block: ModuleBlock,
    pub extends_block: ExtendsBlock,
    pub define_block: DefineBlock,
    pub variable_block: VariableBlock,
    pub invariant_block: InvariantBlock,
}

pub struct ModuleBlock {

}

pub struct ExtendsBlock {

}

pub struct DefineBlock {

}

pub struct InvariantBlock {

}

pub struct VariableBlock {

}

/// Syntax: Op(arg1, arg2) == Expr
pub struct Operator {

}
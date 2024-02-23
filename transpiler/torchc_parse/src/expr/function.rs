use torchc_cgen::cgen;
use torchc_diagnosis::Diagnosis;
use torchc_script::Script;

/// It recursively parses the function expression and obtains the _**cgen data**_.
pub fn function(script: &mut Script, diagnosis: &mut Diagnosis<'_>, parent_expr: &mut cgen::Expr) {}

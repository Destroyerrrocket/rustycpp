use crate::utils::stringref::StringRef;
use deriveMacros::CommonAst;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum ModuleDeclaration {
    /// Holds module name
    ExportPrimary(StringRef),
    /// Holds module name
    Primary(StringRef),
    /// Holds module name + partition
    ExportPartition(StringRef, StringRef),
    /// Holds module name + partition
    Partition(StringRef, StringRef),
    /// Holds resolved path
    ModuleHeaderUnit(u64),
    /// Holds resolved path
    Global(u64),
}

impl Display for ModuleDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExportPrimary(arg0) => format!("Export Primary [{arg0}]").fmt(f),
            Self::Primary(arg0) => format!("Primary [{arg0}]").fmt(f),
            Self::ExportPartition(arg0, arg1) => format!("ExportPartition [{arg0}, {arg1}]").fmt(f),
            Self::Partition(arg0, arg1) => format!("Partition [{arg0}, {arg1}]").fmt(f),
            Self::ModuleHeaderUnit(arg0) => "ModuleHeaderUnit".fmt(f),
            Self::Global(arg0) => "Global".fmt(f),
        }
    }
}

#[derive(Clone, Copy, CommonAst)]
pub struct AstTu {
    #[AstToString]
    moduleUnitKind: ModuleDeclaration,
}

impl AstTu {
    pub fn new_dont_use() -> Self {
        Self {
            moduleUnitKind: ModuleDeclaration::Global(0),
        }
    }
}

use crate::{
    ast::common::AstDecl,
    parse::parser::ModuleImportState,
    utils::structs::{CompileError, CompileMsgImpl, SourceRange},
};

use super::super::Parser;

impl Parser {
    /**
     * Act on module fragment introductor.
     * global-module-fragment:
     *   module-keyword ; declaration-seq [opt]
     *
     * private-module-fragment:
     *   module-keyword : private ; declaration-seq [opt]
     *
     * module-declaration:
     *   export-keyword [opt] module-keyword module-name module-partition [opt] attribute-specifier-seq [opt];
     */
    pub fn actOnModuleDecl(
        &mut self,
        isExport: bool,
        moduleName: String,
        modulePartition: Option<String>,
        location: SourceRange,
    ) {
        if !isExport && moduleName.is_empty() && modulePartition.is_none()
        /* global */
        {
            if self.moduleImportState == ModuleImportState::StartFile {
                self.moduleImportState = ModuleImportState::GlobalSection;
            } else {
                self.errors.push(CompileError::fromSourceRange(
                    "Can't start a global module section if it is not at the start of the file",
                    &location,
                ));
            }
        } else if moduleName.is_empty() && modulePartition.as_ref().is_some_and(|p| p == "private")
        {
            if isExport {
                self.errors.push(CompileError::fromSourceRange(
                    "Can't start a private module section marked as export",
                    &location,
                ));
                return;
            }
            if matches!(
                self.moduleImportState,
                ModuleImportState::CodeSection | ModuleImportState::ImportSection
            ) {
                self.moduleImportState = ModuleImportState::PrivateSection;
            } else {
                self.errors.push(CompileError::fromSourceRange(
                    "Can't start a private module section if it is not after you declare a module",
                    &location,
                ));
                return;
            }
        } else if !moduleName.is_empty() {
            if matches!(
                self.moduleImportState,
                ModuleImportState::StartFile | ModuleImportState::GlobalSection
            ) {
                self.moduleImportState = ModuleImportState::ImportSection;

                let initialModuleDecl = self
                    .compilerState
                    .compileUnits
                    .lock()
                    .unwrap()
                    .get(&self.filePath)
                    .unwrap()
                    .moduleKind
                    .module
                    .0;
                #[rustfmt::skip]
                let mismatch = match (initialModuleDecl, isExport, moduleName, modulePartition) {
                    (crate::module_tree::structs::ModuleDeclaration::ExportPrimary(iniModuleName), true, moduleName, None) if iniModuleName.as_ref() == moduleName.as_str() => {false},
                    (crate::module_tree::structs::ModuleDeclaration::Primary(iniModuleName), false, moduleName, None) if iniModuleName.as_ref() == moduleName.as_str() => {false},
                    (crate::module_tree::structs::ModuleDeclaration::ExportPartition(iniModuleName, iniModulePartition), true, moduleName, Some(modulePartition)) if iniModuleName.as_ref() == moduleName.as_str() && iniModulePartition.as_ref() == modulePartition.as_str() => {false},
                    (crate::module_tree::structs::ModuleDeclaration::Partition(iniModuleName, iniModulePartition), false, moduleName, Some(modulePartition)) if iniModuleName.as_ref() == moduleName.as_str() && iniModulePartition.as_ref() == modulePartition.as_str() => {false},
                    _ => {true},
                };
                if mismatch {
                    self.errors.push(CompileError::fromSourceRange(
                        format!(
                            "Module declaration mismatch; First non-macro pass detected a {initialModuleDecl}, but we are now finding something else! Are you using macros in this specific line? That's a no-no for now (There's a chicken-and-egg problem here, as macros can alter modules/imports, but imports can import macros. (It's complicated, so I've decided to not suport macros in module declarations while still allowing importing macros.)), sorry!"
                        ),
                        &location,
                    ));
                    return;
                }
            }
        } else {
            self.errors.push(CompileError::fromSourceRange(
                "Can't start a module section declared as: ".to_owned()
                    + if isExport { "export" } else { "" }
                    + " module "
                    + &moduleName
                    + " "
                    + &modulePartition.map_or(String::new(), |s| ":".to_owned() + &s)
                    + " ",
                &location,
            ));
            return;
        }
    }

    pub fn actOnTopLevelDecl(&mut self, decl: &Vec<&'static AstDecl>) {
        if !decl.is_empty() {
            self.moduleImportState = match self.moduleImportState {
                ModuleImportState::GlobalSection => ModuleImportState::GlobalSection,
                ModuleImportState::ImportSection | ModuleImportState::CodeSection => {
                    ModuleImportState::CodeSection
                }
                ModuleImportState::PrivateSection => ModuleImportState::PrivateSection,
                ModuleImportState::StartFile | ModuleImportState::GlobalFile => {
                    ModuleImportState::GlobalFile
                }
            };
        }
    }
}

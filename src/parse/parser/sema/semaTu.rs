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
    #[allow(clippy::unused_self)] // TODO: REMOVE
    pub fn actOnModuleDecl(
        &mut self,
        _isExport: bool,
        _moduleName: String,
        _modulePartition: Option<String>,
    ) {
        // TODO
    }

    #[allow(clippy::unused_self)] // TODO: REMOVE
    pub fn actOnTopLevelDecl(&mut self, _decl: &Vec<()>) {
        // TODO
    }
}

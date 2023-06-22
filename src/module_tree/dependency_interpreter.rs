//! Interprets dependency instructions to create the nodes of the tree
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::slice;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use crate::compiler::TranslationUnit;
use crate::utils::stringref::ToStringRef;
use crate::utils::structs::{CompileError, CompileMsg, CompileMsgImpl};

use super::structs::{ModuleDeclaration, ModuleOperator, Node};

/// At the start, or after encountering any module; operator, the module fragment import operations are parsed. It returns the next module operator, if any, and the module depenedencies
fn parseGlobalPartOfModuleFile(
    iter: &mut slice::Iter<ModuleOperator>,
) -> Result<(Option<ModuleOperator>, Vec<ModuleDeclaration>), String> {
    let mut imports = vec![];
    loop {
        let op = iter.next();
        match op {
            None => return Ok((None, imports)),
            Some(op) => match op {
                ModuleOperator::Import(module) => {
                    if module.starts_with(':') {
                        return Err(format!(
                            "Can't import partitions in global module. Tried to import: {module}"
                        ));
                    }
                    imports.push(ModuleDeclaration::ExportPrimary(module.to_StringRef()));
                }
                ModuleOperator::ImportHeader(path) => {
                    imports.push(ModuleDeclaration::ModuleHeaderUnit(*path));
                }

                ModuleOperator::ExportModule(_) | ModuleOperator::Module(_) => {
                    return Ok((Some(op.clone()), imports));
                }
            },
        }
    }
}

/// At the first module <name>;, the imports are parsed. It returns the next module operator, if any, and the module depenedencies
fn parseModulePartOfModuleFile(
    iter: &mut slice::Iter<ModuleOperator>,
    mut name: String,
) -> Result<(Option<ModuleOperator>, Vec<ModuleDeclaration>), String>
where
{
    if let Some((n, _)) = name.split_once(':') {
        name = n.to_string();
    }

    let mut imports = vec![];
    loop {
        let op = iter.next();
        match op {
            None => return Ok((None, imports)),
            Some(op) => match op {
                ModuleOperator::Import(module) => {
                    if let Some(module) = module.strip_prefix(':') {
                        imports.push(ModuleDeclaration::ExportPartition(
                            name.to_StringRef(),
                            module.to_StringRef(),
                        ));
                    } else {
                        imports.push(ModuleDeclaration::ExportPrimary(module.to_StringRef()));
                    }
                }
                ModuleOperator::ImportHeader(path) => {
                    imports.push(ModuleDeclaration::ModuleHeaderUnit(*path));
                }

                ModuleOperator::ExportModule(_) | ModuleOperator::Module(_) => {
                    return Ok((Some(op.clone()), imports));
                }
            },
        }
    }
}

/// From the Module operations, generates a `Node` with the module field filled, and extra nodes for header imports. Also returns the resolved dependency names
#[allow(clippy::too_many_lines)]
pub fn generateNode(
    tu: TranslationUnit,
    ops: &[ModuleOperator],
    isModuleHeader: bool,
    genNewArcTable: &mut HashMap<ModuleDeclaration, Arc<(ModuleDeclaration, TranslationUnit)>>,
) -> Result<(Node, Vec<ModuleDeclaration>), Vec<CompileMsg>> {
    let mut err = vec![];

    let mut moduleImports: Vec<ModuleDeclaration> = vec![];
    let mut moduleName: Option<String> = None;
    let mut modulePrivateFound = false;
    let mut moduleIsExport = false;
    let mut explicitGlobalModuleFound = false;

    let mut iter = ops.iter();
    let mut res = parseGlobalPartOfModuleFile(&mut iter)
        .map_err(|err| vec![CompileError::onFile(err, tu)])?;
    while res.0.is_some() {
        moduleImports.extend(res.1.clone());
        match res.0.as_ref().unwrap() {
            ModuleOperator::ExportModule(name) => {
                moduleIsExport = true;

                if name.is_empty() {
                    err.push(CompileError::onFile(
                        "global part can't be exported".to_string(),
                        tu,
                    ));
                    return Err(err);
                }

                if name == ":private" {
                    err.push(CompileError::onFile(
                        format!(
                            ":private part can't be exported on module {}",
                            moduleName.unwrap_or_else(|| "<unknown module name>".to_string())
                        ),
                        tu,
                    ));
                    return Err(err);
                }

                if moduleName.is_some() {
                    err.push(CompileError::onFile(
                        format!("Module name already defined as {}", moduleName.unwrap()),
                        tu,
                    ));
                    return Err(err);
                }
                moduleName = Some(name.to_string());
                res = parseModulePartOfModuleFile(&mut iter, name.to_string())
                    .map_err(|err| vec![CompileError::onFile(err, tu)])?;
            }
            ModuleOperator::Module(name) => {
                if name.is_empty() {
                    if explicitGlobalModuleFound {
                        err.push(CompileError::onFile("global part already defined", tu));
                        return Err(err);
                    }
                    explicitGlobalModuleFound = true;
                    res = parseGlobalPartOfModuleFile(&mut iter)
                        .map_err(|err| vec![CompileError::onFile(err, tu)])?;
                    continue;
                }

                if name != ":private" && moduleName.is_some() {
                    err.push(CompileError::onFile(
                        format!("Module name already defined as {}", moduleName.unwrap()),
                        tu,
                    ));
                    return Err(err);
                } else if name == ":private" {
                    if moduleName.is_none() {
                        err.push(CompileError::onFile(
                            "Private part of a module must be in a named module. Currently on global",
                            tu,
                        ));
                        return Err(err);
                    }
                    if modulePrivateFound {
                        err.push(CompileError::onFile(
                            format!(
                                "Private part of a module already defined in module {}",
                                moduleName.unwrap()
                            ),
                            tu,
                        ));
                        return Err(err);
                    }
                    modulePrivateFound = true;
                    res = parseModulePartOfModuleFile(
                        &mut iter,
                        moduleName.as_ref().unwrap().to_string(),
                    )
                    .map_err(|err| vec![CompileError::onFile(err, tu)])?;
                    continue;
                }
                moduleName = Some(name.to_string());
                res = parseModulePartOfModuleFile(&mut iter, name.to_string())
                    .map_err(|err| vec![CompileError::onFile(err, tu)])?;
            }
            _ => unreachable!(),
        }
    }
    moduleImports.extend(res.1);

    let moduleDecl = moduleName.map_or_else(
        || {
            if isModuleHeader {
                ModuleDeclaration::ModuleHeaderUnit(tu)
            } else {
                ModuleDeclaration::Global(tu)
            }
        },
        |moduleName| {
            if isModuleHeader {
                err.push(CompileError::onFile(
                    "Module headers can't have a module name declared in them...".to_string(),
                    tu,
                ));
                ModuleDeclaration::ModuleHeaderUnit(tu)
            } else if moduleIsExport {
                if moduleName.contains(':') {
                    let (module, partition) = moduleName.split_once(':').unwrap();
                    ModuleDeclaration::ExportPartition(
                        module.to_StringRef(),
                        partition.to_StringRef(),
                    )
                } else {
                    ModuleDeclaration::ExportPrimary(moduleName.to_StringRef())
                }
            } else if moduleName.contains(':') {
                let (module, partition) = moduleName.split_once(':').unwrap();
                ModuleDeclaration::Partition(module.to_StringRef(), partition.to_StringRef())
            } else {
                moduleImports.push(ModuleDeclaration::ExportPrimary(moduleName.to_StringRef()));
                ModuleDeclaration::Primary(moduleName.to_StringRef())
            }
        },
    );
    match genNewArcTable.entry(moduleDecl) {
        Entry::Vacant(res) => {
            let res = res.insert(Arc::new((moduleDecl, tu)));
            if err.is_empty() {
                Ok((
                    Node {
                        module: res.clone(),
                        dependedBy: vec![],
                        dependsOn: HashSet::new(),
                        depth: 0,
                        stepsCompleted: Arc::new(AtomicUsize::new(0)),
                    },
                    moduleImports,
                ))
            } else {
                Err(err)
            }
        }
        Entry::Occupied(error) => {
            err.push(CompileError::onFile(
                format!(
                    "Module {} already defined. Conflicting files: {} and {}",
                    moduleDecl,
                    error.get().1,
                    tu
                ),
                tu,
            ));
            Err(err)
        }
    }
}

type GeneratedEmptyNodes = HashMap<ModuleDeclaration, (Node, Vec<ModuleDeclaration>)>;
type GenNewArcTable = HashMap<ModuleDeclaration, Arc<(ModuleDeclaration, u64)>>;

/// Generates all the necessary `Node` for each TU (might result in some extras from headers) and fills the dependsOn fields.
pub fn generateEmptyNodes(
    translationUnitContent: &mut dyn Iterator<Item = (TranslationUnit, Vec<ModuleOperator>, bool)>,
) -> Result<(GeneratedEmptyNodes, GenNewArcTable), Vec<CompileMsg>> {
    let mut err = vec![];

    let mut generatedEmptyNodes = HashMap::new();
    let mut genNewArcTable = HashMap::new();
    for (tu, op, isModuleFile) in translationUnitContent {
        match generateNode(tu, &op, isModuleFile, &mut genNewArcTable) {
            Ok((node, depends)) => {
                generatedEmptyNodes.insert(node.module.0, (node, depends));
            }
            Err(mut err2) => err.append(&mut err2),
        }
    }
    if !err.is_empty() {
        return Err(err);
    }
    Ok((generatedEmptyNodes, genNewArcTable))
}
/// Generates all the necessary `Node` for each TU (might result in some extras from headers) and fills the dependsOn fields.
pub fn generateNodes(
    translationUnitContent: &mut dyn Iterator<Item = (TranslationUnit, Vec<ModuleOperator>, bool)>,
) -> Result<HashMap<ModuleDeclaration, Node>, Vec<CompileMsg>> {
    let mut err = vec![];
    let mut res = HashMap::new();

    let (mut generatedEmptyNodes, genNewArcTable) = generateEmptyNodes(translationUnitContent)?;

    #[allow(clippy::needless_collect)]
    for module in generatedEmptyNodes.keys().copied().collect::<Vec<_>>() {
        // Check we have everything we need
        //
        // This makes sure that the module itself is in the list of modules we
        // depend on (just in case. While constructing the tree this is not
        // taken into account, this is just for reporting)
        let mut depenedsPlusModule = generatedEmptyNodes.get(&module).unwrap().1.clone();
        depenedsPlusModule.push(module);

        // Lets check that all the modules we depend on are in the project, as we are going to assume that they are in the following steps
        let mut dependsPlusModuleAvailable = generatedEmptyNodes
            .iter_mut()
            .filter_map(|(key, value)| depenedsPlusModule.contains(key).then_some(&mut value.0))
            .collect::<Vec<_>>();

        if dependsPlusModuleAvailable.len() != depenedsPlusModule.len() {
            let missing = depenedsPlusModule
                .into_iter()
                .filter(|key| !generatedEmptyNodes.contains_key(key))
                .collect::<HashSet<_>>();

            err.push(CompileError::onFile(
                format!("Missing modules in the project: {missing:?}"),
                generatedEmptyNodes.get(&module).unwrap().0.module.1,
            ));
            continue;
        }

        // Now remove the module itself from the dependency list! We don't want to depend on ourselves!
        let module = dependsPlusModuleAvailable.swap_remove(
            dependsPlusModuleAvailable
                .iter()
                .position(|x| x.module.0 == module)
                .unwrap(),
        );

        // Now create the depend on/depended by edges of the graph
        for dep in dependsPlusModuleAvailable {
            module
                .dependsOn
                .insert(genNewArcTable.get(&dep.module.0).unwrap().clone());

            dep.dependedBy
                .push(genNewArcTable.get(&module.module.0).unwrap().clone());
        }
    }

    if !err.is_empty() {
        return Err(err);
    }

    for (module, node) in generatedEmptyNodes {
        res.insert(module, node.0);
    }
    Ok(res)
}

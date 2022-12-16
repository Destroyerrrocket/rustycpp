//! Interprets dependency instructions to create the nodes of the tree
use std::collections::{HashMap, HashSet};
use std::slice;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

use crate::compiler::TranslationUnit;
use crate::utils::filemap::FileMap;
use crate::utils::structs::{CompileError, CompileMsg};

use super::structs::{ModuleDeclaration, ModuleOperator, Node};

/// At the start, or after encountering any module; operator, the module fragment import operations are parsed. It returns the next module operator, if any, and the module depenedencies
fn parseGlobalPartOfModuleFile(
    iter: &mut slice::Iter<ModuleOperator>,
    fileMap: &mut Arc<Mutex<FileMap>>,
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
                    imports.push(ModuleDeclaration::ExportPrimary(module.clone()));
                }
                ModuleOperator::ImportHeader(path) => {
                    let mut fileMap = fileMap.lock().unwrap();
                    if !fileMap.hasFileAccess(&path[1..path.len() - 1]) {
                        return Err(format!("Error resolving path to import header {path}"));
                    }
                    let path = fileMap
                        .getAddFile(&path[1..path.len() - 1])
                        .path()
                        .to_string();
                    imports.push(ModuleDeclaration::ModuleHeaderUnit(path));
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
    fileMap: &mut Arc<Mutex<FileMap>>,
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
                            name.clone(),
                            module.to_string(),
                        ));
                    } else {
                        imports.push(ModuleDeclaration::ExportPrimary(module.clone()));
                    }
                }
                ModuleOperator::ImportHeader(path) => {
                    let mut fileMap = fileMap.lock().unwrap();
                    if !fileMap.hasFileAccess(&path[1..path.len() - 1]) {
                        return Err(format!("Error resolving path to import header {path}"));
                    }
                    let path = fileMap
                        .getAddFile(&path[1..path.len() - 1])
                        .path()
                        .to_string();
                    imports.push(ModuleDeclaration::ModuleHeaderUnit(path));
                }

                ModuleOperator::ExportModule(_) | ModuleOperator::Module(_) => {
                    return Ok((Some(op.clone()), imports));
                }
            },
        }
    }
}

/// From the Module operations, generates a `Node` with the module field filled, and extra nodes for header imports. Also returns the resolved dependency names
pub fn generateNode(
    (tu, ops): (TranslationUnit, Vec<ModuleOperator>),
    genNewArcTable: &mut HashMap<ModuleDeclaration, Arc<(ModuleDeclaration, TranslationUnit)>>,
    fileMap: &mut Arc<Mutex<FileMap>>,
) -> Result<(Vec<Node>, Vec<ModuleDeclaration>), Vec<CompileMsg>> {
    let mut err = vec![];
    let mut nodes = vec![];

    let mut moduleImports: Vec<ModuleDeclaration> = vec![];
    let mut moduleName: Option<String> = None;
    let mut modulePrivateFound = false;
    let mut moduleIsExport = false;
    let mut explicitGlobalModuleFound = false;

    let mut iter = ops.iter();
    let mut res = parseGlobalPartOfModuleFile(&mut iter, fileMap).map_err(|err| {
        vec![CompileError::on_file(
            err,
            fileMap.lock().unwrap().getFile(&tu),
        )]
    })?;
    while res.0.is_some() {
        moduleImports.extend(res.1.clone());
        match res.0.as_ref().unwrap() {
            ModuleOperator::ExportModule(name) => {
                moduleIsExport = true;

                if name.is_empty() {
                    err.push(CompileError::on_file(
                        "global part can't be exported".to_string(),
                        fileMap.lock().unwrap().getFile(&tu),
                    ));
                    return Err(err);
                }

                if name == ":private" {
                    err.push(CompileError::on_file(
                        format!(
                            ":private part can't be exported on module {}",
                            moduleName.unwrap()
                        ),
                        fileMap.lock().unwrap().getFile(&tu),
                    ));
                    return Err(err);
                }

                if moduleName.is_some() {
                    err.push(CompileError::on_file(
                        format!("Module name already defined as {}", moduleName.unwrap()),
                        fileMap.lock().unwrap().getFile(&tu),
                    ));
                    return Err(err);
                }
                moduleName = Some(name.to_string());
                res = parseModulePartOfModuleFile(&mut iter, name.to_string(), fileMap).map_err(
                    |err| {
                        vec![CompileError::on_file(
                            err,
                            fileMap.lock().unwrap().getFile(&tu),
                        )]
                    },
                )?;
            }
            ModuleOperator::Module(name) => {
                if name.is_empty() {
                    if explicitGlobalModuleFound {
                        err.push(CompileError::on_file(
                            "global part already defined",
                            fileMap.lock().unwrap().getFile(&tu),
                        ));
                        return Err(err);
                    }
                    explicitGlobalModuleFound = true;
                    res = parseGlobalPartOfModuleFile(&mut iter, fileMap).map_err(|err| {
                        vec![CompileError::on_file(
                            err,
                            fileMap.lock().unwrap().getFile(&tu),
                        )]
                    })?;
                    continue;
                }

                if name != ":private" && moduleName.is_some() {
                    err.push(CompileError::on_file(
                        format!("Module name already defined as {}", moduleName.unwrap()),
                        fileMap.lock().unwrap().getFile(&tu),
                    ));
                    return Err(err);
                } else if name == ":private" {
                    if moduleName.is_none() {
                        err.push(CompileError::on_file(
                            "Private part of a module must be in a named module. Currently on global",
                            fileMap.lock().unwrap().getFile(&tu),
                        ));
                        return Err(err);
                    } else {
                        if modulePrivateFound {
                            err.push(CompileError::on_file(
                                format!(
                                    "Private part of a module already defined in module {}",
                                    moduleName.unwrap()
                                ),
                                fileMap.lock().unwrap().getFile(&tu),
                            ));
                            return Err(err);
                        }
                        modulePrivateFound = true;
                        res = parseModulePartOfModuleFile(
                            &mut iter,
                            moduleName.as_ref().unwrap().to_string(),
                            fileMap,
                        )
                        .map_err(|err| {
                            vec![CompileError::on_file(
                                err,
                                fileMap.lock().unwrap().getFile(&tu),
                            )]
                        })?;
                        continue;
                    }
                }
                moduleName = Some(name.to_string());
                res = parseModulePartOfModuleFile(&mut iter, name.to_string(), fileMap).map_err(
                    |err| {
                        vec![CompileError::on_file(
                            err,
                            fileMap.lock().unwrap().getFile(&tu),
                        )]
                    },
                )?;
            }
            _ => unreachable!(),
        }
    }
    moduleImports.extend(res.1);

    for op in &moduleImports {
        if let ModuleDeclaration::ModuleHeaderUnit(path) = op {
            if let Ok(res) =
                genNewArcTable.try_insert(op.clone(), Arc::new((op.clone(), path.clone())))
            {
                nodes.push(Node {
                    module: res.clone(),
                    dependedBy: vec![],
                    dependsOn: HashSet::new(),
                    depth: 0,
                    stepsCompleted: Arc::new(AtomicUsize::new(0)),
                });
            }
        }
    }

    let moduleDecl = moduleName.map_or_else(
        || ModuleDeclaration::Global(tu.clone()),
        |moduleName| {
            if moduleIsExport {
                if moduleName.contains(':') {
                    let (module, partition) = moduleName.split_once(':').unwrap();
                    ModuleDeclaration::ExportPartition(module.to_string(), partition.to_string())
                } else {
                    ModuleDeclaration::ExportPrimary(moduleName)
                }
            } else if moduleName.contains(':') {
                let (module, partition) = moduleName.split_once(':').unwrap();
                ModuleDeclaration::Partition(module.to_string(), partition.to_string())
            } else {
                moduleImports.push(ModuleDeclaration::ExportPrimary(moduleName.clone()));
                ModuleDeclaration::Primary(moduleName)
            }
        },
    );

    match genNewArcTable.try_insert(
        moduleDecl.clone(),
        Arc::new((moduleDecl.clone(), tu.clone())),
    ) {
        Ok(res) => {
            nodes.push(Node {
                module: res.clone(),
                dependedBy: vec![],
                dependsOn: HashSet::new(),
                depth: 0,
                stepsCompleted: Arc::new(AtomicUsize::new(0)),
            });
        }
        Err(error) => {
            err.push(CompileError::on_file(
                format!(
                    "Module {} already defined. Confilcting files: {} and {}",
                    moduleDecl, error.value.1, tu
                ),
                fileMap.lock().unwrap().getFile(&tu),
            ));
            return Err(err);
        }
    }

    if err.is_empty() {
        Ok((nodes, moduleImports))
    } else {
        Err(err)
    }
}

/// Generates all the necessary `Node` for each TU (might result in some extras from headers) and fills the dependsOn fields.
pub fn generateNodes(
    ops: Vec<(TranslationUnit, Vec<ModuleOperator>)>,
    fileMap: &mut Arc<Mutex<FileMap>>,
) -> Result<HashMap<ModuleDeclaration, Node>, Vec<CompileMsg>> {
    let mut err = vec![];
    let mut res = HashMap::new();

    let mut generatedEmptyNodes = HashMap::new();
    let mut genNewArcTable = HashMap::new();
    for op in ops {
        match generateNode(op, &mut genNewArcTable, fileMap) {
            Ok((mut nodes, depends)) => {
                let node = nodes.pop().unwrap();
                generatedEmptyNodes.insert(node.module.0.clone(), (node, depends));
                for node in nodes {
                    generatedEmptyNodes.insert(node.module.0.clone(), (node, vec![]));
                }
            }
            Err(mut err2) => err.append(&mut err2),
        }
    }
    if !err.is_empty() {
        return Err(err);
    }

    for module in generatedEmptyNodes
        .keys()
        .cloned()
        .into_iter()
        .collect::<Vec<_>>()
    {
        let mut depenedsPlusModule = generatedEmptyNodes.get(&module).unwrap().1.clone();
        depenedsPlusModule.push(module.clone());

        let mut depends = generatedEmptyNodes
            .iter_mut()
            .filter_map(|(key, value)| depenedsPlusModule.contains(key).then_some(&mut value.0))
            .collect::<Vec<_>>();

        if depends.len() != depenedsPlusModule.len() {
            let missing = depenedsPlusModule
                .into_iter()
                .filter(|key| !generatedEmptyNodes.contains_key(key))
                .collect::<HashSet<_>>();

            err.push(CompileError::on_file(
                format!("Missing modules: {missing:?}"),
                fileMap
                    .lock()
                    .unwrap()
                    .getFile(&generatedEmptyNodes.get(&module).unwrap().0.module.1),
            ));
            continue;
        }

        let module =
            depends.swap_remove(depends.iter().position(|x| x.module.0 == module).unwrap());

        for dep in depends {
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

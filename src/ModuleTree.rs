//! Finds the module related operations, translates them to custom tokens, and
//! generates the module dependency tree.
mod DependencyAnnotate;
mod DependencyDfs;
mod DependencyInterpreter;
pub mod DependencyIterator;
pub mod DependencyParser;
pub mod Generate;
pub mod Structs;

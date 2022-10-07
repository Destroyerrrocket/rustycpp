module;
// I am aware that this is not a valid module declaration, but for creating the dependency tree, I'm going to be significantly more lenient.
// This will be squashed later on in the compilation.
export module foo : otherfunc &&otherwrongtokens||->*<=>=<=+-%^&|~!@#$?/.,;:[]{}()=internal;
void otherfunc();
module : private; // I am aware that this is not valid, but I'm choosing to ignore it
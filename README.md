# C++20ish preprocessor lexer written in Rust

## Currently, a custom build of lalrpop is needed. I'll find a proper solution to this...

This is a very simple and most certainly wrong lexer for preprocessing tokens of C++. This was not done with any major intents, it was simply to test Rust and its capabilities (I find learning by doing a lot more useful than just following tutorials).

As a rust novice, I do not claim this to be of any quality.

So far, the most rellevant missing things are:
- Module tokens are not implemented
- String literal rule is way too lax. This is due to the limitations of the regex engine logos uses (Although I could revalidate the string in a following action. So, still a TODO). This does not cause a missclasification of token, so it's ok.
- If this were to be a compiler, you know, it should accually apply a preprocessing step and then compile :D

So far, I'd say that the first 3 steps of the compilation process of C++ are done-ish! Time for preprocessing.

### Update 26/09/2022:
I took on the project again. The "compiler" is now able to perform macro expansions, but no proper conditional inclusion of code, and certainly not inclusion. For the time being, I do not plan to include any pragmas or useful extensions that other implementations provide :)
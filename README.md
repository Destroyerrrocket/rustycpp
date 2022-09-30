# C++20 preprocessor of C++ written in Rust

## Currently, a custom build of lalrpop is needed. I'll find a proper solution to this, or replace it with antlr, which is now supported.

This is a very simple and most certainly wrong preprocessor for C++. This was not done with any major intents, it was simply to test Rust and its capabilities (I find learning by doing a lot more useful than just following tutorials).

As a rust novice, I do not claim this to be of any quality.

So far, the most rellevant missing things are:
- Module tokens are not implemented in any way. I'm waiting to getting some field experience before even looking at implementing it.
- String literal rule is way too lax. This is due to the limitations of the regex engine logos uses (Although I could revalidate the string in a following action. So, still a TODO). This does not cause a missclasification of token, so it's ok.
- the #line directive is not supported (and probably won't for some time, I do not intend to support generated code for now :) )
- QOL Features of preprocessors, like any pragma directive (none mandated by standard, but #pragma once is expected from any sensible implementation), or the `__FUNCTION__` macro (which requires the step 7 parser to be implemented in order to know such information)
- Most test macros are kinda useless right now. `__has_cpp_attribute` is literally just hardcoded to 0

I'd say that the first 4 steps of the compilation process of C++ are done-ish! Time for lexing.

If you want more logs on what's going on, you can use the environment varaible `RUST_LOG`, like so: `RUST_LOG=debug`

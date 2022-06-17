# C++20ish preprocessor lexer written in Rust

This is a very simple and most certainly wrong lexer for preprocessing tokens of C++. This was not done with any major intents, it was simply to test Rust and its capabilities (I find learning by doing a lot more useful than just following tutorials).

As a rust novice, I do not claim this to be of any quality whatsoever. You should be able to achieve the same (and probably more successfully) in less than an afternoon.

So far, the most rellevant missing things are:
- Module tokens are not implemented
- String literal rule is way too lax. This is due to the limitations of the regex engine logos uses (Although I could revalidate the string in a following action. So, still a TODO)
- If this were to be a compiler, you know, it should accually apply a preprocessing step and then compile :D

So far, I'd say that the first 3 steps of the compilation process of C++ are done-ish! Time for preprocessing
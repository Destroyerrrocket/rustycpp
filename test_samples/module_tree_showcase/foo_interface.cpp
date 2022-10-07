module;
#include <iostream>
void illegalfunctionthatwewontdetect() {
  std::cout << "Hello World!\n";
  module.not.a.real.module();
  export.not.a.real.export();
  import.not.a.real.import();
}
export module foo;
export import :otherfunc;
import :	internal;
export void foo();

module : private;
void privFunc() {
	internal();
	otherfunc();
}
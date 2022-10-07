module;
export module bar;
import <hello.h>
import <importable_header.hpp>
export void hello() {
	HELLO(hello);
	std::cout << "Hello World!\n";
	if (GOOD_INCLUSION) {
		std::cout << "Good inclusion!\n";
	}
}
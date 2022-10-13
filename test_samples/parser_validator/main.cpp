void f() {
	int(a)->m = 7; // expression-statement
	int(a)++; // expression-statement
	int(a,5)<<c; // expression-statement
	int(*d)(int); // declaration
	int(e)[5]; // declaration
	int(f) = { 1, 2 }; // declaration
	int(*g)(double(3));
}
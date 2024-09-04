# Learn the Basics

- ##### ~~User input/output~~ 
	- (std::cout << )  && (std::cin)

- ##### ~~Data Types~~
	- float
	- int
	- unsigned int
	- long long
	- char (primitive) **Written in single quotes 'c'**
	- **FROM STD LIB**
	- string (provided by std library) **Written in double quotes  "string"**
	- vector (just an array could be of any type user defined or primitve)

- #### ~~If else statements and 
	```cpp
	if(true) {
		// do something
	} else {
		// do some other thing
	}
	```

- #### ~~Switch statement
	```cpp
	switch(_expression_) {  
	  case x:
		  break;  
	  case y:
		  break;  
	  default:
		  break;
	  }
	```

- #### ~~For loops~~
	```cpp
	for (int i = 0; i < 10; i++) {
		std::cout << i << std::endl;
	}
	```

- #### ~~While loops~~
	```cpp
	// first checks the condition then executes the loop body
	while(true) {
		// do something
	}

	// very similar do while loops
	do {
		// do something
	} while(true) // checks the condition after the loop body is executed once
	```

- #### ~~Arrays and strings~~
	- string used with double quotes ""
		```cpp
		string name = "harsh""
		```
	- array is a pointer to a memory location in the system, array is a contiguous block of memory.

- #### ~~functions~~
	```cpp
	int myfunction(int parameterNubmer, string parameterString) {
		// do something with the parameter
		return 69; // can only return a number since the return type is int in                        the function signature
	}

	```
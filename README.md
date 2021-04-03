# VSL Programming Language
## Project Goal
The purpose of this project is to create a new programming language that focuses on security to protect against mistakes that may lead to the release of confidential information. This new language will introduce a feature called “variable security.” The idea behind variable security is that there are certain data sets within a program that may be more sensitive than others. Some languages such as C++ or Java allow for the programmer to declare private and public variables within a class, however, variable security takes this a step further and allows the programmer to introduce multiple levels of security priorities for variables. This means that variables could potentially have hierarchies of security within the same program and within the same function. These hierarchies will define how certain variables can interact within one another. The new programming language will be developed by first creating a stack based virtual machine and then writing a compiler that takes a high-level readable language and translates it to the byte-code of the virtual machine. The virtual machine will be developed as a 64 bit machine to allow the language to take full advantage of modern hardware and operating systems. Other than the variable security feature, the language will have a small feature set including functions, loops, if / if else statements, input / output capabilities, integers, floats, strings, and arrays. The intent of this language is to provide companies a cross platform programming language that makes handling sensitive or confidential information intuitive and easy for developers to limit data leaks.

## How to Install and Use
### Windows
1. Download the Windows binaries from releases.
2. Download Windows terminal from the Microsoft Store (Makes instruction comparable to other operating systems).
3. Create a vsl file such as 'main.vsl'.
4. Enter either some example code or code for a new program.
5. In the directory of the binary files and source file type './vsl_compiler main.vsl' in your terminal to compile program.
6. In the directory of the binary files and source file type './vsl_vm program' in your terminal to run the program.

### Linux
1. Download the Linux binaries from releases.
3. Create a vsl file such as 'main.vsl'.
4. Enter either some example code or code for a new program.
5. In the directory of the binary files and source file type './vsl_compiler main.vsl' in your terminal to compile program.
6. In the directory of the binary files and source file type './vsl_vm program' in your terminal to run the program.

### Mac
1. Download the Mac binaries from releases.
3. Create a vsl file such as 'main.vsl'.
4. Enter either some example code or code for a new program.
5. In the directory of the binary files and source file type './vsl_compiler main.vsl' in your terminal to compile program.
6. In the directory of the binary files and source file type './vsl_vm program' in your terminal to run the program.

### Install and Use Tutorial Video
[Tutorial Video](https://youtu.be/8Jw80VwTHbQ)

## Sample Code
### Hello World
```typescript
fn void main() {
    print("Hello, World!\n");
    return;
}
```
### Factorial
```typescript
fn void main() {
    print("Enter an integer below 20: \n");
    let fact int:0 = read();
    print("Factorial of ", fact, ": ", factorial(fact), "\n");
    return;
}

fn int:0 factorial(fact int:0) {
    if fact <= 1 {
        return 1;
    }
    return fact * factorial(fact - 1);
}
```
### Leibniz formula for calculating Pi
```typescript
fn void main() {
    let n int:0 = 100000000;
    let pi float:0 = 4.0;
    let num float:0 = 4.0;
    let den float:0 = 3.0;

    while n >= 0 {
        num = num * -1.0;
        pi = pi + (num / den);
        den = den + 2.0;
        n = n - 1;
    }
    print(pi, "\n");
    return;
}
```
### Fibonacci
```typescript
fn void main() {
    let n int:0 = 20;
    print("Fibonacci of ", n, ": ", fibonacci(n), "\n");
    return;
}

fn int:0 fibonacci(n int:0) {
    let newfib int:0 = 0;
    if n <= 1 {
        return 0;
    } else if n == 2 {
        return 1;
    } else {
        let fib0 int:0 = 0;
        let fib1 int:0 = 1;
        n = n - 2;

        while n >= 0 {
            newfib = fib0 + fib1;
            fib0 = fib1;
            fib1 = newfib;
            n = n - 1;
        }
    }
    return newfib;
}
```
### Print statement
```typescript
fn void main() {
    let language_name string:0 = "Variable" + " Security" + " Language";
    let year int:0 = 2021;
    let creator_name string:0 = "Caden Miller";
    let random_number float:0 = 24.65;

    print("The ", language_name, " was created by ", creator_name, " in ", year, ", and supports floats such as ", random_number, ".\n");
    return;
}
```
### Read statement
```typescript
fn void main() {
    print("What is your name?\n");
    let name string:0 = read();

    print("\nHow old are you?\n");
    let age int:0 = read();

    print("\nNice to meet you ", name, ", who is ", age, " years old.\n");
    return;
}
```
### Break statement
```typescript
fn void main() {
    let n int:0 = 10;
    let i int:0 = 10;

    while n >= 0 {
        print(n, "\n");
        n = n - 1;
        if n == 4 {
            while i >= 0 {
                print(i, "\n");
                i = i - 1;
                if i == 4 {
                    break;
                }
            }
            break;
        }
    }
    return;
}
```
### Expressions
```typescript
fn void main() {
    let n int:0 = 10;
    let i int:0 = 10;

    let t float:0 = 3.2;
    let z float:0 = 4.7;

    let o string:0 = "B4e";
    let q string:0 = "b4e";

    if n == i and t <= z and o != q or 1 == 1 {
        print("This is true\n");
    } else {
        print("This is false\n");
    }
    return;
}
```
### Variable Security Hierarchy
```typescript
fn void main() {
    let sale_id int:0 = 1456328;
    let name string:0 = "John Doe";
    let credit_card_number int:1 = 8879419156;
    let social_security_number int:2 = 784986548;
    return;
}
```

## Language Rules
1. All code must be contained within a function.
2. All functions must return within the root scope of the function (even void functions with 'return;').
3. All variables passed to functions will be passed by value (execpt for arrays).
4. All statements except if and while loops must end with a semicolon.
5. The syntax for defining new variables is as follows:
    let_keyword identifier variable_type:security_value = expression.

## Standard Library
1. ```typescript
    print(arg1, arg2, arg3, ...)
    ```
    The print function takes 1 or more arguments of any primitive type and prints to the screeen.
2. ```typescript
    read()
    ```
    The read function gets input from the user and parses it to the correct type. If this fails for int or float, it returns 0. If it fails for the string type, it returns an empty string.
3. ```typescript
    string_to_int(arg1 string:100)
    ```
    Takes a string as input and converts to an integer. Returns 0 if the conversion fails.
4. ```typescript
    string_to_foat(arg1 string:100)
    ```
    Takes a string as input and converts to a float. Returns 0 if the conversion fails.
5. ```typescript
    int_to_float(arg1 int:100)
    ```
    Takes an integer as input and converts to a float. Returns 0 if the conversion fails.
5. ```typescript
    int_to_string(arg1 int:100)
    ```
    Takes an integer as input and converts to a string. Returns an empty string if the conversion fails.
6. ```typescript
    float_to_int(arg1 float:100) 
    ```
    Takes a float as input and converts to an integer. Returns 0 if the conversion fails.
7. ```typescript
    float_to_string(arg1 float:100)
    ```
    Takes a float an input and converts to a string. Returns an empty string if the conversion fails.
8. ```typescript
    get_string_index(arg1 string:100, arg2 int:100)
    ```
    Takes a string and an integer as input and returns the index of the string as a string. Returns an empty string if fails. arg1 is the original string and arg2 is the index.
9. ```typescript
    set_string_index(arg1 string:100, arg2 int:100, arg3 string:100)
    ```
    Takes a string, an integer, and another string as input. arg1 is the string to be changed, arg2 is the index of the string to change, and arg3 is the value that will be inserted at the index of arg1. 
10. ```typescript
    get_copy_string(arg1 string:100)
    ```
    Takes a string as input and returns a copy of the string.


## To Do List
1. Implement arrays (or vectors) for the int, float, and string data types. (And char indexing for string type)
2. Implement garbage collection in the VM.
3. Add documentation and clean up code.
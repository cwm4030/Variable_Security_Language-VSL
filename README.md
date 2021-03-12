# VSL_Programming_Language

## Project Goal
The purpose of this project is to create a new programming language that focuses on security to protect against mistakes that may lead to the release of confidential information. This new language will introduce a feature called “variable security.” The idea behind variable security is that there are certain data sets within a program that may be more sensitive than others. Some languages such as C++ or Java allow for the programmer to declare private and public variables within a class, however, variable security takes this a step further and allows the programmer to introduce multiple levels of security priorities for variables. This means that variables could potentially have hierarchies of security within the same program and within the same function. These hierarchies will define how certain variables can interact within one another. The new programming language will be developed by first creating a stack based virtual machine and then writing a compiler that takes a high-level readable language and translates it to the byte-code of the virtual machine. The virtual machine will be developed as a 64 bit machine to allow the language to take full advantage of modern hardware and operating systems. Other than the variable security feature, the language will have a small feature set including functions, loops, if / if else statements, input / output capabilities, integers, floats, strings, and arrays. The intent of this language is to provide companies a cross platform programming language that makes handling sensitive or confidential information intuitive and easy for developers to limit data leaks.

## How to Install and Use

### Windows
1. Download the windows binaries from releases.
2. Download windows terminal from the Microsoft Store (The default cmd and powershell applications are notoriously awful).
3. Create a vsl file such as 'main.vsl'
4. Enter either some example code or code for a new program.
5. In the directory of the binary files and source file type './vsl_compiler main.vsl' in your terminal to compile program.
6. In the directory of the binary files and source file type './cmvm program' in your terminal to run the program.

### Linux
1. Download the linux binaries from releases.
3. Create a vsl file such as 'main.vsl'
4. Enter either some example code or code for a new program.
5. In the directory of the binary files and source file type './vsl_compiler main.vsl' in your terminal to compile program.
6. In the directory of the binary files and source file type './cmvm program' in your terminal to run the program.

### Example

[Example Video](https://youtu.be/I007YlARifE)

## Sample Code

### Hello World
```rust
fn void main() {
    print("Hello, World!\n");
    return;
}
```

### Factorial
```rust
fn void main() {
    print("Factorial of 14: ", factorial(14), "\n");
    return;
}

fn int:0 factorial(fact int:0) {
    if fact <= 1 {
        return 1;
    }
    return fact * factorial(fact - 1);
}
```

# VSL_Programming_Language

## Sample Code

### Hello World
```
fn void main() {
    print("Hello, World!\n");
}
```

### Factorial
```
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

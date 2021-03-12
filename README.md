# VSL_Programming_Language

## Sample Code

### Hello World
```typescript
fn void main() {
    print("Hello, World!\n");
    return
}
```

### Factorial
```typescript
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

## Name Notes

If there isn't any handling, can use `expect("add err msg")` or `unwrap()`. Unwrap does not take a err message, and therefore less useful.

## Result Handling

Err() and Ok() are the two results. There are different ways to handle them.

In reality this works with any enum, but is used mainly for enums that contain values.

In this example only Err() returns a value. Therefore we use `if-let` that executes only if the variable is assigned.
A form of Enum destructuring.

```rs
if let Err(error) = io::stdout().flush() {
    println!("{}", error);
    return;
}
```

If Err() and Ok() returns value, then a way to act on both of them is to use a match statement. Need a arm for every option in the Enum, although it can be empty `=>{}`.

```rs
match io::stdout().flush() {
    Ok(thing) => {//this errors because flush has no Ok() value
        println!("{}", thing);
    }
    Err(error) => {
        println!("{}", error);
        return;
    }
}
```

You can also use `let-else`, which assigns a variable if Ok() but if Err() executes code (that must either return or panic).

```rs
let Some(i)= name.find("test") else {
    println!();
    return;
};
```

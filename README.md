# Lunar

Lunar is a secure wrapper around the LUA library for Rust.

The idea is to create a safe and easy-to-implement abstraction, without having to deal with the complex and confusing functions of the LUA library.

# Wiki

### Creating a virtual machine

You need a Lunar virtual machine to create Lua objects and call them.

```rust
use lunar::{lunar::Lunar, value::LunarValue};

extern crate lunar;

fn main() {
    let lunar = Lunar::new(); // Instantiate a Lunar virtual machine.
    lunar.load_std_library(); // is called to load standard LUA libraries, like 'print()'
    lunar.load_str("print('Hello World!')"); // loads a code snippet.
    lunar.exec(); //executes the loaded string.
}

```

---

### Creating a static function

When creating a static function, it will be available in the global namespace of your virtual machine and can be accessed from anywhere.
To create a static function you must call ``create_static_function()`` from a Lunar VM.

The parameters are: 
    ``create_static_function(&self, name: &str, function: fn(ctx: LunarContext) -> i32)``

```rust
use lunar::{lunar::Lunar, value::LunarValue};

extern crate lunar;

fn main() {
    let lunar = Lunar::new();
    lunar.load_std_library();

    lunar.create_static_function("sun", |ctx| {
        let x = ctx.get_integer::<i32>(1).unwrap(); // 3
        let y = ctx.get_integer::<i32>(2).unwrap(); // 5

        return ctx.returns(LunarValue::INTEGER((x + y).into()));
    });


    lunar.load_str(
r#"
    let result = sun(3, 5) -- (x, y)
    print(result) -- 20
"#);

    lunar.exec();
}
```

Note that the function parameter is a callback that takes a context.

A context is a LUA object, which belongs to that current moment, it's like the scope of your object.
Then we pass a lambda expression to get that context and receive the parameters of our function.

Here we see the calls ```ctx.get_integer::<i32>(1)``` and ```ctx.get_integer::<i32>(2)``` from the context of our function.

The calls made get the values ​​passed to our function in numerical order.
Note that we are calling an ``unwrap()`` here. Context functions return a ``Result<T, LunarError>``.
The main reason for this is that LUA, being a dynamically typed language, it may happen that the value passed to the function is not what we expect.

Since we know exactly that the value passed was of type int, we forcefully unwrap the value. But attention, in a real scenario you should implement a safe way to solve this, and not forcing the unwrap using ``unwrap()``.

## License

This project is licensed under the [MIT License](LICENSE)

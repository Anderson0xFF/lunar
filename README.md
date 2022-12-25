# lunar

Secure wrapper around the Lua C library for Rust.

## Examples

### Creating a virtual machine

You need a Lunar virtual machine to create Lua objects and call them.

```rust
use lunar::{lunar::Lunar, value::LunarValue};

extern crate lunar;

fn main() {
    let lunar = Lunar::new();
    lunar.load_std_library();
    lunar.load_str(
r#"
    print("Hello World!)
"#);
    lunar.exec();
}

```

---

### Creating a static function

- When creating a static function, it will be available in the global namespace of your virtual machine and can be accessed from anywhere.

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

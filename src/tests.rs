#[cfg(test)]
mod tests {
    use crate::{
        context::{Userdata, Value},
        lunar::Lunar,
    };

    #[test]
    fn lunar_state_is_valid() {
        //let lunar = Lunar::new();
        //assert!(!lunar.get().is_null());
    }

    #[test]
    fn lunar_state_dropped() {
        // let mut lunar = Lunar::new();
        // lunar.destroy();
        // assert!(lunar.get().is_null());
    }

    #[test]
    fn create_static_function_lua_test() {
        let lunar = Lunar::new();
        lunar.load_std_library();

        lunar.create_static_function("test", |ctx| {
            assert!(true);
            return ctx.returns(Value::Nil);
        });

        lunar.load("test()");
        lunar.exec().unwrap();
    }

    #[test]
    fn static_function_get_args_integer() {
        let lunar = Lunar::new();
        lunar.load_std_library();

        lunar.create_static_function("test", |ctx| {
            assert_eq!(ctx.get_int::<i32>(1), Ok(75));
            assert_eq!(ctx.get_int::<i32>(2), Ok(1166));

            return ctx.returns(Value::Nil);
        });

        lunar.load("test(75, 1166)");
        lunar.exec().unwrap();
    }

    #[test]
    fn static_function_get_args_string() {
        let lunar = Lunar::new();
        lunar.load_std_library();

        lunar.create_static_function("test", |ctx| {
            assert_eq!(ctx.get_string(1), Ok("Hello World!".to_string()));

            ctx.returns(Value::Nil)
        });

        lunar.load("test('Hello World!')");
        lunar.exec().unwrap();
    }

    #[test]
    fn static_function_return_nil() {
        let lunar = Lunar::new();
        lunar.load_std_library();

        lunar.create_static_function("test", |ctx| {
            return ctx.returns(Value::Nil);
        });

        lunar.load("assert(test() == nil)");
        assert_eq!(lunar.exec(), Ok(()));
    }

    #[test]
    fn create_table_empty() {
        let lunar = Lunar::new();
        lunar.load_std_library();

        lunar.create_table("Table", true, |_table| {});

        lunar.load("assert(Table)");
        assert_eq!(lunar.exec(), Ok(()));
    }

    #[test]
    fn create_table_field_int() {
        let lunar = Lunar::new();
        lunar.load_std_library();

        lunar.create_table("Table", true, |table| {
            table.set("value", Value::Int(25));
        });

        lunar.load("assert(Table.value == 25)");
        assert_eq!(lunar.exec(), Ok(()));
    }

    #[test]
    fn create_table_field_string() {
        let lunar = Lunar::new();
        lunar.load_std_library();

        lunar.create_table("Table", true, |table| {
            table.set("value", Value::String(String::from("Hello Lunar!")));
        });

        lunar.load("assert(Table.value == 'Hello Lunar!')");
        assert_eq!(lunar.exec(), Ok(()));
    }

    #[test]
    fn create_userdata() {
        struct Calculator(i32, i32);

        let lunar = Lunar::new();
        lunar.load_std_library();

        lunar.register_userdata("Calculator", |methods| {
            methods.constructor(|ctx| -> i32 {
                let calc = Userdata::new(Calculator(35, 25));
                ctx.returns(Value::Userdata("Calculator", calc.as_ptr(), calc.size()))
            });
        });

        lunar.load(
            "
            local calc = Calculator()
            assert(calc)
            ",
        );

        assert_eq!(lunar.exec(), Ok(()));
    }

    #[test]
    fn create_userdata_get_args() {
        struct Calculator(i32, i32);

        let lunar = Lunar::new();
        lunar.load_std_library();

        lunar.register_userdata("Calculator", |methods| {
            methods.constructor(|ctx| {
                let x = ctx.get_int::<i32>(2).unwrap();
                let y = ctx.get_int::<i32>(3).unwrap();
                let calc = Userdata::new(Calculator(x, y));
                ctx.returns(Value::Userdata("Calculator", calc.as_ptr(), calc.size()))
            });
        });

        lunar.load(
            "
            local calc = Calculator(10, 10)
            assert(calc)
            ",
        );

        assert_eq!(lunar.exec(), Ok(()));
    }

    #[test]
    fn create_userdata_methods() {
        #[derive(Debug)]
        struct Calculator(i32, i32);

        let lunar = Lunar::new();
        lunar.load_std_library();

        lunar.register_userdata("Calculator", |methods| {
            methods.constructor(|ctx| {
                let x = ctx.get_int::<i32>(2).unwrap();
                let y = ctx.get_int::<i32>(3).unwrap();
                let calc = Userdata::new(Calculator(x, y));

                ctx.returns(Value::Userdata("Calculator", calc.as_ptr(), calc.size()))
            });

            methods.add_method("sun", |ctx| {
                let calc = ctx.get_userdata::<Calculator>(1);
                ctx.returns(Value::Int(calc.0 + calc.1))
            });
        });

        lunar.load(
            "
            local calc = Calculator(10, 10)
            local value = calc:sun()
            assert(value == 20)
            ",
        );

        assert_eq!(lunar.exec(), Ok(()));
    }
}

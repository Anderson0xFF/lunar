#[cfg(test)]
mod tests {
    use crate::{state::LunarState, value::LunarValue, vm::Lunar};

    #[test]
    fn lunar_state_is_valided() {
        let lunar = LunarState::new();
        assert!(!lunar.get().is_null());
    }

    #[test]
    fn lunar_state_dropped() {
        let mut lunar = LunarState::new();
        lunar.destroy();
        assert!(lunar.get().is_null());
    }

    #[test]
    fn create_static_function_lua_test() {
        let lunar = Lunar::new();
        lunar.load_std_library();

        lunar.create_static_function("test", |ctx| {
            assert!(true);
            return ctx.returns(LunarValue::NIL);
        });

        lunar.load_str("test()");
        lunar.exec().unwrap();
    }

    #[test]
    fn static_function_get_args_interger() {
        let lunar = Lunar::new();
        lunar.load_std_library();

        lunar.create_static_function("test", |ctx| {
            assert_eq!(ctx.get_integer::<i32>(1), Ok(75));
            assert_eq!(ctx.get_integer::<i32>(2), Ok(1166));

            return ctx.returns(LunarValue::NIL);
        });

        lunar.load_str("test(75, 1166)");
        lunar.exec().unwrap();
    }

    #[test]
    fn static_function_get_args_string() {
        let lunar = Lunar::new();
        lunar.load_std_library();

        lunar.create_static_function("test", |ctx| {
            assert_eq!(ctx.get_string(1), Ok("Hello World!".to_string()));

            return ctx.returns(LunarValue::NIL);
        });

        lunar.load_str("test('Hello World!')");
        lunar.exec().unwrap();
    }

    #[test]
    fn static_function_return_nil() {
        let lunar = Lunar::new();
        lunar.load_std_library();

        lunar.create_static_function("test", |ctx| {
            return ctx.returns(LunarValue::NIL);
        });

        lunar.load_str("assert(test() == nil)");
        assert_eq!(lunar.exec(), Ok(()));
    }

    #[test]
    fn create_table_empty(){
        let lunar = Lunar::new();
        lunar.load_std_library();
        
        lunar.create_table("Table", true, |_table|{
            
        });

        lunar.load_str("assert(Table)");
        assert_eq!(lunar.exec(), Ok(()));
    }

    
    #[test]
    fn create_table_field_int(){
        let lunar = Lunar::new();
        lunar.load_std_library();
        
        lunar.create_table("Table", true, |table|{
            table.set("value", LunarValue::INTEGER(25));
        });

        lunar.load_str("assert(Table.value == 25)");
        assert_eq!(lunar.exec(), Ok(()));
    }

    #[test]
    fn create_table_field_string(){
        let lunar = Lunar::new();
        lunar.load_std_library();
        
        lunar.create_table("Table", true, |table|{
            table.set("value", LunarValue::STRING(String::from("Value String")));
        });

        lunar.load_str("assert(Table.value == 'Value String')");
        assert_eq!(lunar.exec(), Ok(()));
    }
}

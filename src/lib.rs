pub mod context;
mod lua;
pub mod lunar;
pub mod metatable;
mod refr;
mod state;
pub mod table;
pub mod types;

#[cfg(test)]
mod tests {

    use crate::{
        context::{Userdata, Value},
        lunar::Lunar,
    };

    struct Object(i32);
    impl Userdata for Object {}

    #[test]
    fn it_works() {
        let lunar = Lunar::new();
        lunar.load_std_library();
        lunar.create_static_function("hello", |_ctx| {
            println!("Hello World!");
            _ctx.returns(Value::Bool(true))
        });

        lunar.load_string("print('oi')".to_string());
        lunar.exec().unwrap();
    }

    #[test]
    fn create_table() {
        let lunar = Lunar::new();
        lunar.load_std_library();
        
        // lunar.create_table("Person", true, |table| {
        //     table.set("__p", Value::Function(|L|->i32{
        //         return 0;
        //     }));
        // });

        // lunar.create_static_function("awk", |ctx| {
        //     let ctx = Rc::new(ctx);
        //     let table = Table::from(ctx.clone(), "Person");
        //     println!("Table: {}", table.get_int::<i32>("__p").unwrap());

        //     table.set("value", Value::Int(32));

        //     ctx.returns(Value::Nil)
        // });

        lunar.create_userdata("Object", |methods| {
            methods.constructor(|ctx| {
                let obj = Object(10);
                
                ctx.returns(Value::Userdata("Object", obj.as_ptr(), std::mem::size_of::<Object>(),
                ))
            });

            methods.add_method("getValue", |ctx|{

                ctx.returns(Value::Int(1))
            });
            

        });
        let script = include_str!("../example/example.lua");
        lunar.load_string(script.to_string());
        lunar.exec().unwrap();
    }
}

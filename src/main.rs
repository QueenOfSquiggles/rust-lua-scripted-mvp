use std::{
    env,
    fs::{DirBuilder, File},
    io::Read,
};

use mlua_extras::{
    extras::Module,
    mlua::{self, prelude::*},
    typed::{
        generator::{Definition, DefinitionFileGenerator, Definitions},
        TypedModule,
    },
};

const LUA_SRC_DIR: &'static str = "src_lua";

fn main() {
    // generate definitions file
    if env::args().any(|a| a.to_lowercase().contains("def")) {
        println!("Attempting to emit definitions files");
        let defs = Definitions::start()
            .define(
                "init",
                Definition::start()
                    .register_module::<RpgModule>("rpg")
                    .finish(),
            )
            .finish();

        let _ = DirBuilder::new().create(LUA_SRC_DIR);
        for (name, writer) in DefinitionFileGenerator::new(defs).iter() {
            writer
                .write_file(&format!("{LUA_SRC_DIR}/{name}"))
                .expect(format!("Failed to emit definitions file {name}").as_str());
            println!("Emitted definitions file: {name}");
        }
    }

    let lua = mlua::Lua::new();
    lua.globals()
        .set("rpg", RpgModule::module())
        .expect("Failed to register module");
    let mut file =
        File::open(format!("{LUA_SRC_DIR}/main.lua")).expect("Failed to open script file");
    let mut buffer: String = "".into();
    file.read_to_string(&mut buffer)
        .expect("Failed to read file");
    match lua.load(buffer).eval::<mlua::Value>() {
        Ok(value) => {
            println!("Script returned: {:?}", value);
        }
        Err(err) => {
            println!("Script error: {err}");
        }
    };
}

struct RpgModule;

impl TypedModule for RpgModule {
    fn documentation() -> Option<String> {
        Some("A module for some basic rpg logic".into())
    }

    fn add_methods<'lua, M: mlua_extras::typed::TypedModuleMethods<'lua>>(
        methods: &mut M,
    ) -> mlua_extras::mlua::Result<()> {
        methods.document("doc").add_function_with(
            "create_rpg_char",
            do_nothing::<String>,
            |func| {
                func.param(0, |p| {
                    p.set_doc("A parameter!").set_name("name");
                });
            },
        )?;

        Ok(())
    }
}

fn do_nothing<T>(_ctx: &Lua, val: T) -> mlua_extras::mlua::Result<T> {
    Ok(val)
}

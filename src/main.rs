use std::{
    borrow::Cow,
    env,
    fs::{DirBuilder, File},
    io::Read,
};

use mlua_extras::{
    extras::Module,
    mlua::{self, prelude::*, Value},
    typed::{
        generator::{Definition, DefinitionFileGenerator, Definitions, FunctionBuilder},
        IntoDocComment, Type, TypedModule, TypedMultiValue, TypedUserData,
    },
    Typed, UserData,
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
                    .module::<RpgModule>("rpg")
                    .register::<RpgCharacter>("RpgCharacter")
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
    } else {
        let lua = mlua::Lua::new();
        lua.globals()
            .set("rpg", RpgModule::module())
            .expect("Failed to register module");
        let mut file =
            File::open(format!("{LUA_SRC_DIR}/main.lua")).expect("Failed to open script file");
        let mut buffer: String = "".into();
        file.read_to_string(&mut buffer)
            .expect("Failed to read file");

        match lua.load(buffer).eval::<Value>() {
            Ok(value) => {
                println!("Script returned: {value:?}");
            }
            Err(err) => {
                panic!("Script error: {err}");
            }
        };
    }
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
            create_rpg_character,
            |func| {
                func.param(0, |p| {
                    p.doc("The name for the character").name("name");
                    p
                });
            },
        )?;

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Typed, UserData)]
struct RpgCharacter {
    name: String,
}

impl<'lua> FromLua<'lua> for RpgCharacter {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let Some(table) = value.as_table() else {
            return Err(LuaError::FromLuaConversionError {
                from: "Unknown",
                to: "RpgCharacter",
                message: Some(format!("failed to convert {value:?} to type RpgCharacter")),
            });
        };
        let Ok(name): LuaResult<String> = table.get("name") else {
            return Err(LuaError::runtime(""));
        };
        Ok(RpgCharacter { name })
    }
}

impl TypedUserData for RpgCharacter {
    fn add_fields<'lua, F: mlua_extras::typed::TypedDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field("name", "");
    }

    fn add_methods<'lua, T: mlua_extras::typed::TypedDataMethods<'lua, Self>>(methods: &mut T) {
        // TODO: It appears there is no way to create a method with no parameters

        methods.document("Provides the current character's attack");
        methods.add_method("get_attack", |_ctx, _this, _p: ()| Ok(1.0));

        methods.document("Testing many parameters");
        methods.add_method_with(
            "long_method",
            |_, _, _: (f32, bool, String, i32, [f32; 3])| Ok(""),
            |doc: &mut FunctionBuilder<_, &str>| {
                document_params(doc, vec![
                    ("float_value", Type::number(), "A cute little float number"),
                    ("truth", Type::boolean(), "Is this true? Maybe maybe not"),
                    ("fun_text", Type::string(), "What fun. What will you write??"),
                    ("second_number", Type::integer(), "This one's an integer!!!"),
                    ("a_vector", Type::tuple([Type::number(), Type::number(), Type::number()]), "Even a tuple can be fun!"),                
                ]);
            },
        );
        // Type::integer()

        methods.document("A testing function to explore various options of API systems");
        methods.add_method_with(
            "test",
            |_ctx, _this, _p: ()| Ok(1.0),
            |a| {
                a.ret(0, |r| r.doc("1.0 a float value, staticly bound to `1.0`"));
                a.document("This function provides some kind of testing for the character");
            },
        );

        // Note that this does require an instance of RpgCharacter since Lua has no actual knowledge of classes, let along `RpgCharacter` specifically
        methods.document("Constructor");
        methods.add_function("new", |_, p: (String,)| {
            Ok(RpgCharacter {name: p.0})
        });

        methods.add_meta_method(LuaMetaMethod::ToString, |_, b, _: Value| {
            Ok(format!("{b:?}"))
        });
    }

    fn add_documentation<F: mlua_extras::typed::TypedDataDocumentation<Self>>(docs: &mut F) {
        docs.add("An RPG character");
    }
}

fn create_rpg_character(_ctx: &Lua, name: String) -> mlua_extras::mlua::Result<RpgCharacter> {
    Ok(RpgCharacter { name })
}

fn document_params<Params, Return, N, D>(
    func_doc: &mut FunctionBuilder<Params, Return>,
    doc_data: Vec<(N, Type, D)>,
) where
    Params: TypedMultiValue,
    Return: TypedMultiValue,
    N: Into<Cow<'static, str>> + Clone,
    D: IntoDocComment + Clone,
{
    // Might want to contribute some kind of helper like this back to mlua-extras crate
    for (index, (name, ptype, doc)) in doc_data.iter().enumerate() {
        full_param_doc(func_doc, index, name.clone(), ptype.clone(), doc.clone());
    }
}

fn full_param_doc<Params, Return, N, D>(
    func_doc: &mut FunctionBuilder<Params, Return>,
    index: usize,
    name: N,
    ptype: Type,
    doc: D,
) where
    Params: TypedMultiValue,
    Return: TypedMultiValue,
    N: Into<Cow<'static, str>> + Clone,
    D: IntoDocComment + Clone,
{
    func_doc.param(index, move |pdoc| {
        pdoc.name(name.clone()).ty(ptype.clone()).doc(doc.clone())
    });
}

// fn register_rust_method<'lua, T, P>(methods: &mut impl TypedDataMethods<'lua, T>, rust_func : Fn<&Lua, &T, P>) {}

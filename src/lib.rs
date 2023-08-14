use mlua::prelude::*;
pub fn apply(lua: &Lua) -> LuaResult<()> {
    lua.globals().set("_hello", "kissa")?;
    lua.load(r#"console.log("hello",_hello)"#).exec()?;
    Ok(())
}

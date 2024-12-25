-- testing a system which would handle creating RPG data through Lua Scripting

local char = rpg.create_rpg_char("testing")
print("Lua found " .. char)

return char

-- testing a system which would handle creating RPG data through Lua Scripting
-- the idea is the script would serve as a factory pattern for elements used on the Rust side

-- `rpg` is a registered global with the associated function
-- the `init.d.lua` script does not provide logic. It's merely a promise of functionality
local char = rpg.create_rpg_char("testing")

print("Lua found " .. tostring(char))
print("" .. char.name .. " with attack: " .. tostring(char:get_attack()))


char:test()
char:test()

print("Testing edit:: " .. char.name .. " with attack: " .. tostring(char:get_attack()))

return char

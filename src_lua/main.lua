-- testing a system which would handle creating RPG data through Lua Scripting
-- the idea is the script would serve as a factory pattern for elements used on the Rust side

-- `rpg` is a registered global with the associated function
-- the `init.d.lua` script does not provide logic. It's merely a promise of functionality
local char = rpg.create_rpg_char("testing")

print("Lua found " .. tostring(char))
print("\twith attack: " .. tostring(char:get_attack()))

char:test()
char:long_method( -- Go ahead, try messing with the parameters. You'll get static analysis to show if you mess something up!
    0.1,
    false,
    "Something",
    0,
    {
        -- tuples can be passed this way, expanding to make it a bit cleaner
        1.0, 2.0, 3.0
    }
)

return char

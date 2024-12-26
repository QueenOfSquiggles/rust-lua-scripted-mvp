--- @meta

--- A module for some basic rpg logic
rpg = {
  --- doc
  --- @param name string The name for the character
  --- @return RpgCharacter 
  create_rpg_char = function(name) end,
}

--- An RPG character
--- @class RpgCharacter
--- @field name string
local _CLASS_RpgCharacter_ = {
  --- Provides the current character's attack
  --- @param self RpgCharacter
  --- @return number 
  get_attack = function(self) end,
  --- A testing function to explore various options of API systems
  --- @param self RpgCharacter
  --- @return number 1.0 a float value, staticly bound to `1.0`
  test = function(self) end,
  __metatable = {
    --- @param self RpgCharacter
    --- @param param0 any 
    --- @return string 
    __tostring = function(self, param0) end,
  }
}


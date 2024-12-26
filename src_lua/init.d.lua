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
  --- Constructor
  --- @param param0 string 
  --- @return RpgCharacter 
  new = function(param0) end,
  --- Provides the current character's attack
  --- @param self RpgCharacter
  --- @return number 
  get_attack = function(self) end,
  --- Testing many parameters
  --- @param self RpgCharacter
  --- @param float_value number A cute little float number
  --- @param truth boolean Is this true? Maybe maybe not
  --- @param fun_text string What fun. What will you write??
  --- @param second_number integer This one's an integer!!!
  --- @param a_vector [number, number, number] Even a tuple can be fun!
  --- @return string 
  long_method = function(self, float_value, truth, fun_text, second_number, a_vector) end,
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


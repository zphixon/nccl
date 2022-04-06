local numlines = tonumber(arg[1]) or 65535

local function generateName()
  local letters = {
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
  }

  local name = ''
  for _ = 1, 10 do
    local letter = letters[math.random(1, #letters)]
    name = name .. letter
  end

  return name
end

local currentIndent = 0
local function printIndent()
  for _ = 1, currentIndent do
    io.write('  ')
  end
end
local function modifyIndent()
  local r = math.random(1, 2)
  if r == 1 then
    currentIndent = currentIndent + 1
  else
    currentIndent = currentIndent - 1
    if currentIndent < 0 then
      currentIndent = 0
    end
  end
end

for _ = 1, numlines do
  printIndent()
  modifyIndent()
  print(generateName())
end

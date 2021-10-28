-- algorithm for parsing the token stream

inspect = require('inspect').inspect
i = 1; function consume() i = i + 1 end
function parse(tokens, indent, parent)
  local node = {key=tokens[i], value={}}; consume()
  while tokens[i] == indent + 1 do
    consume(); parse(tokens, indent + 1, node)
  end
  table.insert(parent.value, node)
end

top = {key = '__top__', value={}}

big = {
  'sandwich',
    1, 'meat',
      2, 'bologna',
      2, 'ham',
    1, 'cheese',
      2, 'provolone',
      2, 'cheddar',
  'cow',
    1, 'one',
  'sub',
  'with',
  'prime',
    1, 'ibschi',
      2, 'gauri',
        3, 'toggi'
}

while i <= #big do
  parse(big, 0, top)
end

print(inspect(top))

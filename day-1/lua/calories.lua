

local highest1 = 0
local highest2 = 0
local highest3 = 0
local sum = 0



for line in io.lines("inputs.txt") do 
  if line == "" then
    if sum > highest1 then
      highest3 = highest2
      highest2 = highest1
      highest1 = sum
    elseif sum > highest2 then
      highest3 = highest2
      highest2 = sum
    elseif sum > highest3 then
      highest3 = sum
    end
    sum = 0
  else
    local num = tonumber(line)
    sum = sum + num
  end
end

if sum > highest1 then
  highest3 = highest2
  highest2 = highest1
  highest1 = sum
elseif sum > highest2 then
  highest3 = highest2
  highest2 = sum
elseif sum > highest3 then
  highest3 = sum
end

print("Highest is:", highest1)
print("Total of highest 3:", highest1 + highest2 + highest3)
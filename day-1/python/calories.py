
highest_1 = 0
highest_2 = 0
highest_3 = 0
sum = 0

file = open('inputs.txt', 'r')
lines = file.readlines()

for line in lines:
    if line == "\n":
        if sum > highest_1:
            highest_3 = highest_2
            highest_2 = highest_1
            highest_1 = sum
        elif sum > highest_2:
            highest_3 = highest_2
            highest_2 = sum
        elif sum > highest_3:
            highest_3 = sum

        sum = 0;
        continue

    num = int(line)
    sum += num

if sum > highest_1:
    highest_3 = highest_2
    highest_2 = highest_1
    highest_1 = sum
elif sum > highest_2:
    highest_3 = highest_2
    highest_2 = sum
elif sum > highest_3:
    highest_3 = sum

print("Highest is: ", highest_1)
print("Total of highest 3:", highest_1 + highest_2 + highest_3)


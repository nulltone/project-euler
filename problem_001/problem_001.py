max_numbers = 1000
total = 0

for i in xrange(1, max_numbers):
  if (i % 3) == 0 or (i % 5) == 0:
    total += i
    continue

print("Total = " + str(total))

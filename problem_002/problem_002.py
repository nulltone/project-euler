total = 0
fibb1 = 1
fibb2 = 1
fibb = 1

while fibb < 4000000:
  if fibb % 2 == 0:
    total += fibb
  fibb = fibb1 + fibb2
  fibb2 = fibb1
  fibb1 = fibb

print("Total = " + str(total))

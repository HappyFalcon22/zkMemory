
num = 0xabc0040a
slice = 0x100

high = num // slice
low = num % slice

num = [1] * 32
slice = 17
cell = 8

# Write (14, 0xffaabbcceedd3388)
temp = slice % cell
chunk = 0xffaabbcceedd3388
low = slice - temp
high = low + cell
print(high, low)
low_write = chunk // (1 << (8 * (temp)))
high_write = chunk % (1 << (8 * (temp)))
print("low : 0x{:016x}".format(low_write))
print("high : 0x{:016x}".format(high_write << ((cell - temp) * 8)))
import random

tries = 1000      # Times the algorithm will be ran.
ncoupons = 50     # Number of coupons.
boxeslist = []    # List of opnened boxes per try.

expected = 0.0  # Expected number of boxed to get all coupons.
mean = 0.0      # Mean number of boxes opened in all tries.

for i in range (1, ncoupons + 1):
    expected += 1.0 / float(i)
    
expected *= ncoupons

for t in range (0, tries):
    boxes = 0                     # Number of opened boxes.
    coupons = range (0, ncoupons) # List of coupons.
    
    while coupons != []:
        boxes += 1
        coupon = random.randint(0, ncoupons)
        
        if coupon in coupons:
            coupons.remove(coupon)

    boxeslist.append(boxes)

for b in boxeslist:
    mean += b

print("La cantidad media de intentos fue " + str(int(mean / tries)) + ".")    
print("La cantidad de intentos esperada era " + str(int(expected)) + ".")


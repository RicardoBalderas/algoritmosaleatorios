import random

coefs = [-  98896140000.0, - 285056496000.0, - 247671152550.0, -  23271127065.0,
            48717850707.0,     8138130777.0, -   2628372129.0, -    242157818.0,
               48345358.0,        1851858.0, -       335880.0, -         5277.0,
                    975.0,              5.0, -            1.0]
roots = [   1.0, - 15.0, -  6.0,    3.0,    7.0,   17.0, - 12.0,
         - 19.0,    1.0,    1.0, -  3.0,   20.0,   15.0, - 15.0]
sign = -1.0;
done = 'n';

custom = raw_input("Valores (1) predeterminados o (2) propios: ")
tries = int(raw_input("Numero de ejecuciones del algoritmo: "))
factor = float(raw_input("Factor: "))
digits = int(raw_input("Cantidad de digitos de redondeo: "))

if custom == '2':
    power = 0
    
    coefs = []
    roots = []
    
    while done == 'n':
        coef = float(raw_input("Inserta el coeficiente de x^" + str(cdeg) + ": "))    
        coefs.append(coef);
        done = raw_input("Has terminado? [s/n]: ");
        power = power + 1
        
    done = 'n'
        
    while done == 'n':
        neg = raw_input("X es negativa? [s/n]: ");
        root = float(raw_input("Inserta la constante del binomio (x + c): "))

        if neg == 's':
            sign = -sign
            root = -root
            
        roots.append(root)
        done = raw_input("Has terminado? [s/n]: ")

maxm = factor * len(roots)
equal = True;

for t in range(tries):
    x = round(random.random() * (2 * maxm) - maxm, digits)
    
    ra = 0.0
    rb = 1.0
    
    power = 0.0
    
    for c in coefs:
        ra += (c * x**power)
        power += 1.0

    for r in roots:
        rb *= (x + r)

    rb *= sign;
    
    ra = round(ra, digits)
    rb = round(rb, digits)

    print("x = " + str(x) + ", ra = " + '{0:.4f}'.format(ra) + 
          ", rb = " + '{0:.4f}'.format(rb))

    if ra != rb:
        print("Los polinomios son diferentes.")
        equal = False
        break
    
if equal:
    print("Los polinomios son iguales.");


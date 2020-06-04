import random

tries = 40
nodes = 4000
prob = 0.0033

conn = 0;

for t in range(tries):
    lnodes = range(nodes)
    matrix = [[] for i in lnodes]

    openl = [0]

    for r in lnodes:
        for c in range(r + 1, nodes): 
            if (random.random() > (1 - prob)):
                matrix[r].append(c)
                matrix[c].append(r)
                
  # print(matrix)

    while (len(openl) != 0 and len(lnodes) != 0):
        node = openl.pop(0)
      # print(node)
        if (node in lnodes):
            lnodes.remove(node)
            openn = matrix[node]
            openl = openn + openl
        if (len(lnodes) == 0):
            conn = conn + 1
            
print("Connected: " + str(conn * 100 / tries) + "%")


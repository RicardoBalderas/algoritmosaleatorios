import random
import timeit

comparisons = 0

def quicksort(ilist, start, end):
    if start < end:
        idx = order(ilist, start, end)
    
        quicksort(ilist, start, idx - 1)
        quicksort(ilist, idx + 1, end)

def order(ilist, start, end):
    global comparisons
    
    i = start
    idx = random.randint(start, end)
    
    tmp = ilist[idx]
    ilist[idx] = ilist[end]
    ilist[end] = tmp
    
    for j in range(start, end):
        comparisons += 1
        if (ilist[j] < ilist[end]):
            tmp = ilist[i]
            ilist[i] = ilist[j]
            ilist[j] = tmp
            i += 1
            
    tmp = ilist[i]
    ilist[i] = ilist[end]
    ilist[end] = tmp
   
    return i

def main():
    tries = 1000
    items = 50
    mean_time = 0.0
    
    acc = 0.0
    for i in range (1, items + 1):
        acc += (1.0 / float(i))
        
    expected = (2.0 * float(items) + 2.0) * acc - 4 * float(items)
    
    for t in range(0, tries):
        ilist = []

        for i in range(1, items + 1):
            ilist.append(random.randint(0, items * 10))

        start = timeit.default_timer()
        quicksort(ilist, 0, items - 1)
        end = timeit.default_timer()

        mean_time += (end - start)

    print('Tiempo promedio por instancia: {:.10f} segundos.'.format(mean_time / tries))
    print('Comparaciones promedio: {:.2f}.'.format(comparisons / tries))
    print('Valor esperado de comparaciones: {:.2f}.'.format(expected))
    
main()


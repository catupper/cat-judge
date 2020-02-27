from random import randint as rr

def make_input(file_name):
    with open(file_name, "w") as f:
        f.write("{} {}\n".format(rr(0,10**10), rr(0,10**10)))

for i in range(20):
    make_input("{}_{}.txt".format(i//10, i%10))

from z3 import Int, Solver, sat, Sum

def parse_input():
    with open("input.txt") as f:
        machines = []
        for line in f.read().split("\n")[:-1]:
            [_, *buttons, joltages] = line.split(" ")
            buttons = [ [ int(i) for i in 
                          button[1:-1].split(",")
                          ] for button in buttons ]
            joltages = [ int(i) for i in 
                          joltages[1:-1].split(",") ]
            machines.append((buttons, joltages))
    return machines

def solve_machine(buttons, joltages):
    symbols = []
    s = Solver()
    for i in range(len(buttons)):
        symbol = Int("k"+str(i))
        symbols.append(symbol)
        s.add(symbol >= 0)
        
    for i in range(len(joltages)):
        s.add(Sum(symbols[j] for j in range(len(buttons)) if i in buttons[j]) == joltages[i])

    minimum = None
    while s.check() == sat:
        model = s.model()
        solution = {sym: model[sym].as_long() for sym in symbols}
        total = sum(solution.values())        
        minimum = total
        s.add(Sum(*symbols) < minimum)
    return minimum
    

machines = parse_input()

print("Part 2:", sum(solve_machine(*machine) for machine in machines))

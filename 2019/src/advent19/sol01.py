import numpy as np
from .day01 import day01


masses = np.loadtxt("input/01.txt", dtype=int)
print(masses)
print("Total fuel=", day01.total_fuel(masses))

# Extra
print("Total fuel recursive=", day01.total_fuel_rec(masses))

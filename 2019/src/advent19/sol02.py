import numpy as np
from .day02 import day02


def run_intcode(intcode):
    intcode_array = np.asarray(intcode, dtype=np.int32)
    day02.run_intcode(intcode_array)
    return intcode_array


if __name__ == "__main__":
    intcode = np.loadtxt("input/02.txt", dtype=int, delimiter=',')

    # restore the gravity assist program (your puzzle input) to the "1202
    # program alarm" state
    intcode[1] = 12
    intcode[2] = 2
    print(run_intcode(intcode))

from numpy.testing import assert_equal
from advent19.sol02 import run_intcode


def test_day02():
    assert_equal(run_intcode([1,0,0,3,99]), [1,0,0,2,99])
    assert_equal(run_intcode([1,0,0,0,99]), [2,0,0,0,99])
    # 1 + 1 = 2).
    assert_equal(run_intcode([2,3,0,3,99]), [2,3,0,6,99])
    # 3 * 2 = 6).
    assert_equal(run_intcode([2,4,4,5,99,0]), [2,4,4,5,99,9801])
    # 99 * 99 = 9801).
    assert_equal(run_intcode([1,1,1,4,99,5,6,0,99]), [30,1,1,4,2,5,6,0,99])

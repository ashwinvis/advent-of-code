from advent19 import day01

ffm = day01.day01.fuel_from_mass
ffmr = day01.day01.fuel_from_mass_rec


def test_day01():
    assert ffm(12) == 2
    assert ffm(14) == 2
    assert ffm(1969) == 654
    assert ffm(100756) == 33583


def test_day01_part2():
    assert ffmr(14) == 2
    assert ffmr(1969) == 966
    assert ffmr(100756) == 50346

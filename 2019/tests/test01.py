def test_day01():
    from advent19 import day01

    ffm = day01.day01.fuel_from_mass

    assert ffm(12) == 2

    assert ffm(14) == 2

    assert ffm(1969) == 654

    assert ffm(100756) == 33583

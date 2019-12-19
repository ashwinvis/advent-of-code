# Advent of Code 2019

https://adventofcode.com/2019

Languages: Python, Fortran, C++

The novelty this year is to do mixed-language computation. Implementation of
the logic would be in Fortran / C++. Python would be only used to do I/O: pass
input and receive output.

## Workflow

### Using f2py

```sh
pip install -e .
cd src && make && cd -
pytest tests/*
```

### Using meson (WIP)
```sh
meson build
cd build/
ninja
```

## Solution

```sh
python -m advent19.sol01
python -m advent19.sol02
...
```

## References

* https://www.fortran90.org/src/best-practices.html
* http://fortranwiki.org/fortran/show/Build+tools
* https://mesonbuild.com/Quick-guide.html
* https://docs.scipy.org/doc/numpy/f2py/getting-started.html

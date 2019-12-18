project('advent', 'fortran')

# Compiler flags
#
fflags = ['-fno-underscoring', '-fno-second-underscore']

add_global_arguments(fflags, language : 'fortran')

sources = files('src/types.f90')
days = ['day01', 'day02']

foreach day : days
    library(day, sources + ['src/' + day + '.f90'])
endforeach

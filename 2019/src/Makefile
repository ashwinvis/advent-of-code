SRC := types.f90

%.pyf: $(SRC) %.f90
	gfortran -c $^
	f2py -m $(basename $@) -h $@ $^

compile: day01.pyf
	$(foreach pyf, $^, \
		f2py -c $(pyf) -m $(basename $(pyf)) $(basename $(pyf)).f90 $(SRC))
	$(foreach pyf, $^, \
		mv -f $(basename $(pyf))*.so advent19/)


clean:
	rm -f *.mod *.o
	rm -f *.pyf *f2pywrappers2.f90

cleanall: clean
	rm -f *.so

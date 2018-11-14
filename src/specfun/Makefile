FC := gfortran
FFLAGS := -O3 -fPIC
OBJ := $(addprefix $(OUT_DIR)/, $(subst .f90,.o,$(shell ls *.f90)))


$(OUT_DIR)/%.o: %.f90
	$(FC) $(FFLAGS) -c $< -o $@

default: $(OBJ)
	ar crus $(OUT_DIR)/libspecfun.a $(OBJ)

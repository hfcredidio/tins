FC := gfortran
FFLAGS := -c -O3 -std=legacy -fPIC -L /usr/local/Cellar/gcc/8.2.0/lib/gcc/8
OBJ := $(addprefix $(OUT_DIR)/, $(subst .f,.o,$(shell ls *.f)))


$(OUT_DIR)/%.o: %.f
	$(FC) $(FFLAGS) $< -o $@

default: $(OBJ)
	ar crus $(OUT_DIR)/libcdflib.a $(OUT_DIR)/*.o

FC := gfortran
FFLAGS := -O3 -fPIC
OBJ := $(addprefix $(OUT_DIR)/, $(subst .f,.o,$(shell ls *.f)))


$(OUT_DIR)/%.o: %.f
	$(FC) $(FFLAGS) -c $< -o $@

default: $(OBJ)
	ar crus $(OUT_DIR)/libamos.a $(OBJ)

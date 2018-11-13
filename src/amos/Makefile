FC := gfortran
FFLAGS := -O3 -fPIC
OBJ := $(addprefix $(OUT_DIR)/, $(subst .f,.o,$(shell ls *.f)))


$(OUT_DIR)/%.o: %.f
	sed -e 's/ZABS/AZABS/' $< > $(OUT_DIR)/_$<
	$(FC) $(FFLAGS) -c $(OUT_DIR)/_$< -o $@

default: $(OBJ)
	ar crus $(OUT_DIR)/libamos.a $(OBJ)

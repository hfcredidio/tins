CC := gcc
CFLAGS := -c -O3 -fPIC
OBJ := $(addprefix $(OUT_DIR)/, $(subst .c,.o,$(shell ls *.c)))


$(OUT_DIR)/%.o: %.c
	$(CC) $(CFLAGS) $< -o $@

default: $(OBJ)
	ar crus $(OUT_DIR)/libcdflib.a $(OUT_DIR)/*.o

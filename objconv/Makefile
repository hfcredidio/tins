FILES=$(shell ls *.cpp)
$(OUT_DIR)/objconv: $(FILES)
	g++ -O2 -o $@ $^

clean:
	rm $(OUT_DIR)/objconv

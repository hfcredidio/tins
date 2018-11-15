FILES=$(shell ls *.cpp)
objconv: $(FILES)
	g++ -O2 -o $(OUT_DIR)/$@ $^

clean:
	rm $(OUT_DIR)/objconv

W2C2 := /home/polygon/repos/w2c2
LDFLAGS += -lm -L${W2C2}/wasi/build -lw2c2wasi
CFLAGS += -O3
W2C2_BIN := ${W2C2}/build/w2c2/w2c2
CC := clang

module: module.o main.o
	$(CC) $^ -O2 -o module $(LDFLAGS)

%.c: %.wasm
	$(W2C2_BIN) $< $@

%.o: %.c
	$(CC) -I${W2C2}/w2c2 -c $(CFLAGS) $< -o $@

.PHONY: clean
clean:
	rm -f *.o module module.c module.h

CARGO := cargo

.PHONY: clean clean-examples

all: lib examples

lib:
	$(CARGO) build

examples: lib
	make -C examples/

clean-examples:
	make -C examples/ clean

clean: clean-examples
	$(RM) -r ./target
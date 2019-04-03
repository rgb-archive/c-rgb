CARGO := cargo

.PHONY: clean clean-examples

-include config.vars

FEATURES=

ifeq ($(WITH_KALEIDOSCOPE),1)
FEATURES += with-kaleidoscope
endif

ifeq ($(WITH_BIFROST),1)
FEATURES += with-bifrost
endif

all: lib examples

lib:
	$(CARGO) build --features "${FEATURES}"

examples: lib
	make -C examples/

clean-examples:
	make -C examples/ clean

clean: clean-examples
	$(RM) -r ./target
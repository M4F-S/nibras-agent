# Makefile - Nibras Sovereign Personal AI Agent
# Track: Best Personal AI Agent | Dubai AI Agents Championship 2026-2027

CC ?= cc
CFLAGS ?= -std=c99 -Wall -Wextra -Werror -O2 -Iadapters
LDFLAGS ?= -lm

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
    LIB_EXT = dylib
    SHARED_FLAGS = -dynamiclib
else
    LIB_EXT = so
    SHARED_FLAGS = -shared
endif

BUILD_DIR = build
LIB = $(BUILD_DIR)/libnibras.$(LIB_EXT)

all: $(BUILD_DIR) $(LIB) $(BUILD_DIR)/test_nibras

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(LIB): adapters/nibras_arabic_fademem_adapter.c adapters/grounding_shield.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -fPIC $(SHARED_FLAGS) $^ $(LDFLAGS) -o $(LIB)

$(BUILD_DIR)/test_nibras: adapters/nibras_arabic_fademem_adapter.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -DNIBRAS_STANDALONE_TEST $^ $(LDFLAGS) -o $@

test: all
	@echo "=================================================================="
	@echo " RUNNING NIBRAS ARABIC NORMALIZER & FADEMEM TEST SUITE"
	@echo "=================================================================="
	./$(BUILD_DIR)/test_nibras
	@echo "[SUCCESS] All Nibras Agent verification suites passed deterministically!"

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all test clean

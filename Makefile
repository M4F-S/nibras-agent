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
AGENT = $(BUILD_DIR)/nibras_agent
TEST_BIN = $(BUILD_DIR)/test_nibras

ADAPTER_SRCS = adapters/nibras_arabic_fademem_adapter.c \
               adapters/grounding_shield.c \
               adapters/nibras_memory_store.c

all: $(BUILD_DIR) $(LIB) $(AGENT) $(TEST_BIN)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(LIB): $(ADAPTER_SRCS) | $(BUILD_DIR)
	$(CC) $(CFLAGS) -fPIC $(SHARED_FLAGS) $^ $(LDFLAGS) -o $(LIB)

$(AGENT): src/nibras_agent.c $(ADAPTER_SRCS) | $(BUILD_DIR)
	$(CC) $(CFLAGS) $^ $(LDFLAGS) -o $@

$(TEST_BIN): tests/test_nibras_suite.c $(ADAPTER_SRCS) | $(BUILD_DIR)
	$(CC) $(CFLAGS) $^ $(LDFLAGS) -o $@

test: all
	@echo "=================================================================="
	@echo " RUNNING NIBRAS ARABIC NORMALIZER & FADEMEM TEST SUITE"
	@echo "=================================================================="
	./$(TEST_BIN)
	@echo "[SUCCESS] All Nibras Agent verification suites passed deterministically!"

clean:
	rm -rf $(BUILD_DIR)
	rm -f data/*.jsonl *.jsonl *.audit.jsonl

.PHONY: all test clean

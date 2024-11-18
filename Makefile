VERTEX_SHADER = shader.vert
VERTEX_SHADER_PATH = ./src/vulkan/pipe/shader/.tmp/triangle/shader.vert
VERTEX_COMPILED = vert.spv
VERTEX_COMPILED_PATH = ./src/vulkan/pipe/shader/.tmp/triangle/vert.spv

FRAGMENT_SHADER = shader.frag
FRAGMENT_SHADER_PATH = ./src/vulkan/pipe/shader/.tmp/triangle/shader.frag
FRAGMENT_COMPILED = frag.spv
FRAGMENT_COMPILED_PATH = ./src/vulkan/pipe/shader/.tmp/triangle/frag.spv

OS := $(shell uname -s)

ifeq ($(OS),Windows_NT)
  GLSLC = glslc.exe
else
  GLSLC = glslc
endif

all: main

main:
	echo "Hola, mundo!";

shaders:
	@echo -e "\n[+] Compiling $(VERTEX_SHADER) to $(VERTEX_COMPILED)"
	$(GLSLC) $(VERTEX_SHADER_PATH) -o $(VERTEX_COMPILED_PATH)
	@echo -e "\n[+] Compiling $(FRAGMENT_SHADER) to $(FRAGMENT_COMPILED)"
	$(GLSLC) $(FRAGMENT_SHADER_PATH) -o $(FRAGMENT_COMPILED_PATH)
	@echo -e "\n[*] done\n"

clean:
	rm -f $(VERTEX_COMPILED_PATH) $(FRAGMENT_COMPILED_PATH)

#.PHONY: all clean
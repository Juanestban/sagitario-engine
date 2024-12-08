VERTEX_SHADER = shader.vert
VERTEX_SHADER_PATH = ./editor/src/vulkan/pipe/shader/.tmp/triangle/shader.vert
VERTEX_COMPILED = vert.spv
VERTEX_COMPILED_PATH = ./editor/src/vulkan/pipe/shader/.tmp/triangle/vert.spv

FRAGMENT_SHADER = shader.frag
FRAGMENT_SHADER_PATH = ./editor/src/vulkan/pipe/shader/.tmp/triangle/shader.frag
FRAGMENT_COMPILED = frag.spv
FRAGMENT_COMPILED_PATH = ./editor/src/vulkan/pipe/shader/.tmp/triangle/frag.spv

OS := $(shell uname -s)

ifeq ($(OS),Windows_NT)
  GLSLC = glslc.exe
else
  GLSLC = glslc
endif

all: main

main:
	@echo "scripts availables:";
	@echo "1	make build-editor";
	@echo "1.1	make shaders";
	@echo "1.2	make run-editor";
	@echo "";
	@echo "2	make build-debugger";
	@echo "2.1	make run-debugger";
	@echo "";
	@echo "3 	make build-profiler";
	@echo "3.1 	make run-profiler";
	@echo "";
	@echo "4	make build-lng";
	@echo "4.1	make run-lng";
	@echo "";
	@echo "5	make clean";

build-editor:
	@echo "building editor...";
	cargo build -p sagitario-editor
	@echo "[+] done ✅"

run-editor:
	@echo "running editor...";
	cargo run -p sagitario-editor
	@echo "[+] done ✅"

shaders:
	@echo -e "\n[+] Compiling $(VERTEX_SHADER) to $(VERTEX_COMPILED)"
	$(GLSLC) $(VERTEX_SHADER_PATH) -o $(VERTEX_COMPILED_PATH)
	@echo -e "\n[+] Compiling $(FRAGMENT_SHADER) to $(FRAGMENT_COMPILED)"
	$(GLSLC) $(FRAGMENT_SHADER_PATH) -o $(FRAGMENT_COMPILED_PATH)
	@echo -e "\n[*] done\n"

build-debugger:
	@echo "building debugger...";
	cargo build -p sagitario-debugger
	@echo "[+] done ✅"

run-debugger:
	@echo "running debugger...";
	cargo run -p sagitario-debugger
	@echo "[+] done ✅"

build-profiler:
	@echo "building profiler...";
	cargo build -p sagitario-profiler
	@echo "[+] done ✅"

run-profiler:
	@echo "running profiler...";
	cargo run -p sagitario-profiler
	@echo "[+] done ✅"

build-lng:
	@echo "[+] building lng...";
	cargo build -p sagitario-lng
	@echo "[+] done ✅"

run-lng:
	@echo "running lng...";
	cargo run -p sagitario-lng
	@echo "[+] done ✅"

clean:
	rm -f $(VERTEX_COMPILED_PATH) $(FRAGMENT_COMPILED_PATH)

#.PHONY: all clean
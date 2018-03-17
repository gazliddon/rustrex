BIN_PATH := ../bin/osx
AS := $(BIN_PATH)/asm6809
OUT := out

all : dirs code
	@echo Done

code : $(OUT)/all.bin $(OUT)/all.syms makefile
	@echo Built code

dirs : $(OUT)
	@echo Made directories

$(OUT) :
	@mkdir -p $(OUT)

$(OUT)/all.bin $(OUT)/all.syms : all.s
	@$(AS) -9 -B -o $(OUT)/all.bin all.s -s $(OUT)/all.syms -v
	@echo Compiled all.s

clean :
	@rm -rf $(OUT)
	@echo cleaned


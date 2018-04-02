BIN_PATH := ../bin/osx
AS := lwasm
OUT := out

SRC := $(wildcard *.s)

all : dirs code
	@echo Done

code : $(OUT)/all.bin $(OUT)/all.syms makefile
	@echo Built code

dirs : $(OUT)
	@echo Made directories

$(OUT) :
	@mkdir -p $(OUT)

$(OUT)/all.bin $(OUT)/all.syms : $(SRC)
	@$(AS) -9 -r -o $(OUT)/all.bin all.s -s -l$(OUT)/all.syms
	@echo Compiled all.s

clean :
	@rm -rf $(OUT)
	@echo cleaned


unpack-stm32f3.tar.gz:
	rm -rf stm32f3
	tar -xzf stm32f3.tar.gz
	rm stm32f3/src/main.c
	@echo "(main.c is not used)"


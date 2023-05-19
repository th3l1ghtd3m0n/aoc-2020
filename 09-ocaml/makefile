INPUTS=example_input.txt

.PHONY: main 

main: main.native main.byte $(INPUTS)
	./response.native $(INPUTS)

main.native: response.ml
	ocamlopt -o response.native response.ml

main.byte: response.ml
	ocamlc -o response.byte response.ml

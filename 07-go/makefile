INPUTS=input1.txt input2.txt

.PHONY: test
test: main $(INPUTS)
	./response $(INPUTS)

main: response.go
	go build -o response response.go

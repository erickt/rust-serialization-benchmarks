all: bench

bench: ffjson goser

.PHONY: ffjson goser

ffjson:
	env GOPATH=`pwd`/gopath PATH=$$PATH:`pwd`/gopath/bin sh -c 'cd gopath/src/github.com/pquerna/ffjson && make bench'

goser:
	cd goser && $(MAKE)

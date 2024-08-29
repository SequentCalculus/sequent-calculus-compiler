.PHONY: paper
paper:
	latexmk $(ROOT_DOC)

.PHONY: clean
clean: 
	@find . -name "*.log" -type f -delete
	@find . -name "*.aux" -type f -delete
	@find . -name "*.out" -type f -delete
	@find . -name "*.bbl" -type f -delete
	@find . -name "*.blg" -type f -delete
	@find . -name "*.xdv" -type f -delete
	@find . -name "*.fdb_latexmk" -type f -delete
	@find . -name "*.fls" -type f -delete

SVG_FILES := $(wildcard img/*.svg)
PDF_FILES := $(SVG_FILES:.svg=.pdf)

.PHONY: pdfs clean

pdfs: $(PDF_FILES)

img/%.pdf: img/%.svg
	inkscape $< --export-type=pdf --export-filename=$@

clean:
	rm -f img/*.pdf

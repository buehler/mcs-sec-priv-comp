SVG_FILES := $(wildcard img/*.svg)
MMD_FILES := $(wildcard img/*.mmd)
PDF_FILES := $(SVG_FILES:.svg=.pdf) $(MMD_FILES:.mmd=.pdf)

.PHONY: pdfs clean

pdfs: $(PDF_FILES)

img/%.pdf: img/%.svg
	inkscape $< --export-type=pdf --export-filename=$@

img/%.pdf: img/%.mmd
	mmdc -t neutral -f -b transparent --input $< --output $@

clean:
	rm -f img/*.pdf

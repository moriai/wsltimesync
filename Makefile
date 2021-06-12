FTIMESYNC = target/release/ftimesync
PROGRAMS = ftimesync wsltimesync
DESTDIR = /usr/local
INSTALL = install

all: build

build: $(PROGRAMS)

install: build
	-@for f in $(PROGRAMS); do \
		echo install $$f; \
		$(INSTALL) $$f $(DESTDIR)/bin; \
	done

ftimesync: $(FTIMESYNC)
	strip -o $@ $^

$(FTIMESYNC): cargo-build
	cargo build --release

.PHONY: cargo-build

clean:
	rm -fr target/release target/debug
	rm -f $(PROGRAMS) timestamp

distclean: clean
	cargo clean

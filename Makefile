TEMPFILE := $(shell mktemp)
csv: src/main.rs
	cargo build --release;
	echo "duration_iterator, duration_while" > ${TEMPFILE}
	for i in `seq 1 15`; do ./target/release/rust_linz; done >>${TEMPFILE};
	sed -i 's/[a-zA-Z]\+: //g' ${TEMPFILE};
	mv ${TEMPFILE} ./csv.csv;
	rm -f ${TEMPFILE};

clean:
	rm -f csv.csv;
	cargo clean

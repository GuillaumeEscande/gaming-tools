
all: test merge

test: test_libs test_src

test_libs: test_solver test_gamio test_logger

test_solver: 
	cd lib/solver && \
	cargo test

test_gamio: 
	cd lib/gamio && \
	cargo test

test_logger: 
	cd lib/logger && \
	cargo test

test_src: test_ch_sring_2021

test_ch_sring_2021:
	cd src/ch_spring_2021 && \
	cargo test

merge: merge_ch_sring_2021

merge_ch_sring_2021:
	mkdir -p target
	./tools/merger.sh -l gamio -l logger -o ./target/ch_sring_2021.rs ch_spring_2021
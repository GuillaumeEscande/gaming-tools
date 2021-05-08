
all: test merge

test: test_libs test_src

test_libs: test_board test_solver test_graph test_gamio test_logger

test_board: 
	cd lib/board && \
	cargo test

test_graph: 
	cd lib/graph && \
	cargo test

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
	./tools/merger.sh -l graph -l gamio -l logger -o ./target ch_spring_2021
	cd target && \
	cargo test
	

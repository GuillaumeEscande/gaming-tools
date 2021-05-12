
all: test merge

test: libs src

libs: board solver graph gamio logger state

board: 
	cd lib/board && \
	cargo test

graph: 
	cd lib/graph && \
	cargo test

solver: 
	cd lib/solver && \
	cargo test

gamio: 
	cd lib/gamio && \
	cargo test

logger: 
	cd lib/logger && \
	cargo test

state: 
	cd lib/state && \
	cargo test

src: ch_sring_2021

ch_sring_2021:
	cd src/ch_spring_2021 && \
	cargo test

merge: merge_ch_sring_2021

merge_ch_sring_2021:
	mkdir -p target
	./tools/merger.sh -l board -l graph -l gamio -l logger -l state -o ./target ch_spring_2021
	cd target && \
	cargo test
	

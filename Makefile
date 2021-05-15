
all: test merge

test: libs src

libs: board game linearhexagon hexagon solver graph gamio logger state 

board: 
	cd lib/board && \
	cargo test

game: 
	cd lib/game && \
	cargo test

linearhexagon: 
	cd lib/linearhexagon && \
	cargo test

hexagon: 
	cd lib/hexagon && \
	cargo test

solver: 
	cd lib/solver && \
	cargo test

graph: 
	cd lib/graph && \
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

src: ch_spring_2021_simu

ch_spring_2021:
	cd src/ch_spring_2021 && \
	cargo test

ch_spring_2021_simu:
	cd src/ch_spring_2021_simu && \
	cargo test

merge: merge_ch_spring_2021_simu

merge_ch_spring_2021:
	mkdir -p target
	./tools/merger.sh -l board -l graph -l gamio -l logger -l state -o ./target ch_spring_2021
	cd target && \
	cargo test

merge_ch_spring_2021_simu:
	mkdir -p target
	./tools/merger.sh -l board -l game -l solver -l linearhexagon -o ./target ch_spring_2021_simu
	cd target && \
	cargo test
	

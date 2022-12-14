
all: test merge

test: libs src

libs: board board2d game linearhexagon hexagon solver graph logger state 

board: 
	cd lib/board && \
	cargo test
	
board2d:
	cd lib/board2d && \
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

logger: 
	cd lib/logger && \
	cargo test

state: 
	cd lib/state && \
	cargo test

src: ch_winter_2022

ch_spring_2021:
	cd src/ch_spring_2021 && \
	cargo test

ch_spring_2021_simu:
	cd src/ch_spring_2021_simu && \
	cargo test

ch_winter_2022:
	cd src/ch_winter_2022 && \
	cargo test

merge: merge_ch_winter_2022

merge_ch_spring_2021:
	mkdir -p target
	./tools/merger.sh -l board -l graph -l logger -l state -o ./target ch_spring_2021
	cd target && \
	cargo test

merge_ch_spring_2021_simu:
	mkdir -p target
	./tools/merger.sh -l board -l game -l solver -l linearhexagon -o ./target ch_spring_2021_simu
	cd target && \
	cargo test

merge_ch_winter_2022: ch_winter_2022
	mkdir -p target
	./tools/merger.sh -l board -l board2d -l graph -l logger -o ./target ch_winter_2022
	cd target && \
	cargo test
	

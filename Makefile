dev : 
	cargo watch -qc  -x 'run' -x 'test --bin=sasha'

test :
	cargo test --bin=sasha

build :
	cargo build

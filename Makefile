# all:
#     $(cargo build)
#     $(sudo setcap cap_net_admin+ep ./target/debug/wg-registry-server)
#
# run:
#     $(./target/debug/wg-registry-server)

all:
	cargo build
	sudo setcap cap_net_admin+ep ./target/debug/wg-registry-server	

run: all
	./target/debug/wg-registry-server

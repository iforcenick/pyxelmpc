cargo build --release --examples -p px-mpc-protocol

./gg20_sm_manager

./gg20_keygen -t 1 -n 3 -i 1 --output local-share1.json
./gg20_keygen -t 1 -n 3 -i 2 --output local-share2.json
./gg20_keygen -t 1 -n 3 -i 3 --output local-share3.json

./gg20_offline -p 1,2 -l local-share1.json
./gg20_offline -p 1,2 -l local-share2.json

./gg20_siggen -p 1,2 -d "Hello" -o offline-stage.json -u 2
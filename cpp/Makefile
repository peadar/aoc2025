all:
	mkdir -p debug
	mkdir -p release
	cmake -S . -B debug -DCMAKE_BUILD_TYPE=Debug
	cmake -S . -B release -DCMAKE_BUILD_TYPE=RelWithDebInfo
	make -C debug
	make -C release

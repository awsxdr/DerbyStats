@ECHO OFF

IF NOT [%1] == [] SET /A build_type = %1
IF [%build_type%] == [] SET /A build_type = "Debug"

cd ui
CALL npm ci
CALL npm run build
cd ..

rmdir /s /q .\build
cmake -B ./build -G "Ninja" -DCMAKE_BUILD_TYPE=%build_type%
cmake --build ./build --config %build_type%
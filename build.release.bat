SETLOCAL
set OUTPUT_FOLDER=target\dist
set EXE_NAME=deck-strategy
echo Output folder: %OUTPUT_FOLDER%\%EXE_NAME%.exe

rmdir target\dist /S /Q
mkdir target\dist
cargo rustc --release -- -C link-args=-Wl,--subsystem,windows

copy target\release\rs-baby-engine.exe %OUTPUT_FOLDER%\%EXE_NAME%.exe
copy *.dll target\dist\
mkdir target\dist\res
xcopy res target\dist\res /E /Y

rcedit %OUTPUT_FOLDER%\%EXE_NAME%.exe --set-icon %OUTPUT_FOLDER%\res\icon.ico
echo Release dist created.
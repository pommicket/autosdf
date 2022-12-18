del autosdf-windows.zip
rd /s/q autosdf
mkdir autosdf
copy /y target\debug\autosdf.exe autosdf
copy README.md autosdf
copy example*.png autosdf
copy settings.txt autosdf
copy SDL2.dll autosdf
7z a autosdf-windows.zip autosdf

cargo build -F console --target i686-pc-windows-msvc
cp target\i686-pc-windows-msvc\debug\d3d9.dll D:\Blur\d3d9.dll
cp target\i686-pc-windows-msvc\debug\d3d9.pdb D:\Blur\d3d9.pdb

#cargo build --release -F gui,console --target i686-pc-windows-msvc
#cp target\i686-pc-windows-msvc\release\d3d9.dll D:\Blur\d3d9.dll

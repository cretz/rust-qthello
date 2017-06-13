## Building on Windows

Steps

* Install latest stable Rust (1.18+)
* Install latest Qt (5.8+) for msvc 64 bit and [2015 C++ build tools](http://landinghub.visualstudio.com/visual-cpp-build-tools)
* Clone somewhere, cd to dir
* Add Qt bin dir to PATH (e.g. C:\Qt\5.8\msvc2015_64\bin)
* Run `vcvarsall.bat amd64` to put 64 bit msvc compiler on path (e.g. `"C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" amd64`)
* Run `cargo build --release`
* From the build release lib dirs, put qt_core_c.dll and qt_widgets_c.dll adjacent to the exe at target/release/qthello.exe. The DLLs will be in target/release/build/libname-<hex>/out/c_lib_install.
* From the Qt bin dir, put Qt5Core.dll, Qt5Gui.dll, and Qt5Widgets.dll also adjacent to the qthello.exe prog

Now you can run the qthello.exe prog.

TODO: automate this in a build.rs script

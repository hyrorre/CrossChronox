# CrossChronox
CrossChronox is a Cross-platform rhythm game based on C++ and OpenGL (SFML2)  
It works on Windows, macOS, and Linux.  
We can play bms, bme, bml, pms, and bmson.  
Read Wiki for more information.

LICENSE : LGPL v3

(WIP)

# Dependencies
* boost  
* picojson  
* SFML  
* sfeMovie  
* Qt  
* Crypto++

# Building on macOS(Xcode)
1. Install boost and Crypto++ with homebrew.
1. Download SFML and sfeMovie, and copy frameworks to /Library/Frameworks
1. Download picojson, and copy picojson.h to /usr/local/include/picojson
1. Install Qt, and make symbolic links. (run following scripts on terminal)
1. Open CrossChronox.xcodeproj and build!

```
sudo ln -s (Qt Application Directory)/(Qt Version)/clang_64/lib/QtCore.framework    /Library/Frameworks
sudo ln -s (Qt Application Directory)/(Qt Version)/clang_64/lib/QtWidgets.framework /Library/Frameworks
sudo ln -s (Qt Application Directory)/(Qt Version)/clang_64/lib/QtGui.framework /Library/Frameworks
```
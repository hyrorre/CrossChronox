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
1. Install boost and Crypto++ using homebrew.
2. Download SFML and sfeMovie, and copy frameworks to /Library/Frameworks
3. Download picojson, and copy picojson.h to /usr/local/include/picojson
4. Install Qt, and make symbolic links. (run following scripts on terminal)
5. Open CrossChronox.xcodeproj and build!

```
sudo ln -s (Qt Application Directory)/(Qt Version)/clang_64/lib/QtCore.framework    /Library/Frameworks
sudo ln -s (Qt Application Directory)/(Qt Version)/clang_64/lib/QtWidgets.framework /Library/Frameworks
sudo ln -s (Qt Application Directory)/(Qt Version)/clang_64/lib/QtGui.framework /Library/Frameworks
```
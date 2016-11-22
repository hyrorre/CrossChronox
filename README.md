# CrossChronox
CrossChronox is a Cross-platform rhythm game based on C++ and OpenGL(SFML). (and Qt)  
It works on Windows, macOS, and Linux.  
We can play bms, bme, bml, pms, and bmson.  
Read Wiki for more information.

LICENSE : LGPL v3

(WIP)

# Dependencies
* boost  
* jsoncpp  
* SFML  
* sfeMovie  
* Qt  
* Crypto++

# Building on macOS
1. Install boost, jsoncpp and Crypto++ using homebrew.
* Download SFML and sfeMovie, and put frameworks to /Library/Frameworks
* Install Qt, and make symbolic links. (run following scripts on terminal)

```
cd (Qt Application Directory)/(Qt Version)/clang_64/lib
ln -s QtWidgets.framework QtCore.framework /Library/Frameworks
```
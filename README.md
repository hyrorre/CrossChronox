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
* SFML2  
* sfeMovie  
* Crypto++

# Building on macOS(Xcode)
1. Install boost and Crypto++ with homebrew.
1. Download SFML2 and sfeMovie, and copy frameworks to /Library/Frameworks
1. Download picojson, and copy picojson.h to /usr/local/include/picojson
1. Open CrossChronox.xcodeproj and build!

# Building on Windows(Visual Studio)
1. Set up environment variable $(INCLUDE) and $(LIB)  
(Library include path and *.lib file path)
1. Install boost, Crypto++, SFML2, and sfeMovie to $(INCLUDE) and &(LIB)  
(You can also install boost, Crypto++ and SFML2 with nuget on Visual Studio)
1. Open CrossChronox.sln and build!
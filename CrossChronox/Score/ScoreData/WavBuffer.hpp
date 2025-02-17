#ifndef WavBuffer_hpp
#define WavBuffer_hpp

#include "pch.hpp"

class WavPlayer;

class WavBuffer{
	friend WavPlayer;
    sf::SoundBuffer buf;
public:
    std::string filename;
    
    WavBuffer(){}
    WavBuffer(const std::string& filename): filename(filename){}
	WavBuffer(const char* filename): filename(filename){}
	bool Load(const std::string& score_directory);
};

#endif /* WavBuffer_hpp */

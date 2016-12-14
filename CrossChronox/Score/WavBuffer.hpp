//
//  WavBuffer.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/10.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef WavBuffer_hpp
#define WavBuffer_hpp

#include "pch.hpp"

class WavBuffer{
    sf::SoundBuffer buf;
public:
    std::string filename;
    
    WavBuffer(){}
    WavBuffer(const std::string& filename): filename(filename){}
	WavBuffer(const char* filename): filename(filename){}
    bool Load(const std::string& score_directory){
        assert(score_directory.back() == '/' || score_directory.back() == '\\');
        return buf.loadFromFile(score_directory + filename);
    }
};

#endif /* WavBuffer_hpp */

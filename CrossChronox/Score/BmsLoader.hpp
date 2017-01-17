//
//  BmsLoader.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 10/7/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#ifndef BmsLoader_hpp
#define BmsLoader_hpp

#include "pch.hpp"
#include "ScoreData.hpp"
#include "Exception.hpp"

void LoadBms(const std::string& path, ScoreData* out, bool load_header_only_flag = false) throw(LoadError, OpenError, ParseError);

#endif /* BmsLoader_hpp */

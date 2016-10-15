//
//  MD5.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/10/14.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef MD5_hpp
#define MD5_hpp

#include "pch.hpp"

bool FileToMD5(const std::string& path, std::string* out);
bool StringToMD5(const std::string& str, std::string* out);
bool StringToMD5(const char* str, std::string* out);
bool StringToMD5(const char* str, std::string* out, size_t len);

#endif /* MD5_hpp */

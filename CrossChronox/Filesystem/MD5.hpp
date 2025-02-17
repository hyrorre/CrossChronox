#pragma once

#include "pch.hpp"

bool FileToMD5(const std::string& path, std::string* out);
bool StringToMD5(const std::string& str, std::string* out);
bool StringToMD5(const char* str, std::string* out);
bool StringToMD5(const char* str, std::string* out, size_t len);

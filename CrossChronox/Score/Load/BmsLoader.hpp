#ifndef BmsLoader_hpp
#define BmsLoader_hpp

#include "pch.hpp"
#include "Score/ScoreData/ScoreData.hpp"
#include "System/Exception.hpp"

void LoadBms(const std::string& path, ScoreData* out, bool load_header_only_flag = false) throw(LoadError, OpenError, ParseError);

#endif /* BmsLoader_hpp */

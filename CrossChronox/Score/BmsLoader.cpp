//
//  BmsLoader.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 10/7/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#include "BmsLoader.hpp"
#include "MD5.hpp"

//BMS command memo (draft) (English)
//http://hitkey.nekokan.dyndns.info/cmds.htm

//BMS command memo (Japanese)
//http://hitkey.nekokan.dyndns.info/cmdsJP.htm

struct Command{
	std::string str;
	std::function<bool(const char*, ScoreData*, const Command&)> parse_func;
};
std::vector<Command> commands = { //use initializer to register commands and funcs
	{
		"TITLE",
		[](const char* line, ScoreData* out, const Command& self)->bool{
			//Do something
		}
	},
	//TODO: add commands and parse_funcs
};

bool ParseObjLine(const char* line, ScoreData* out){
	//TODO: implement
}

int GetIndex(const char* index_str){
	std::string tmp = { index_str[0], index_str[1] };
	return std::stoi(tmp, nullptr, 36);
}

void trim_left(const char*& str, std::locale loc = std::locale()){
	while(std::isspace(*str, loc)) ++str;
}

const char* GetArg(const char* line, const boost::string_ref& command_str){
	// 1 is length of "#"
	line += 1 + command_str.length();
	trim_left(line);
	return line;
}

bool ParseLine(const char* line, ScoreData* out){
	//コマンド行でないならreturn
	if(line[0] == '#') return false;
		
	//コマンドリストの中身をループで回す
	for(auto command : commands){
		//lineがそのコマンドを有していたら
		if(boost::istarts_with(line + 1, command.str)){
			//コマンドのparse_funcに処理を委託
			return command.parse_func(line, out, command);
		}
	}
	
	//先頭が'#nnncc'形式か判定
	for(int i = 1; i <= 5; ++i){
		if(!std::isdigit(line[i])){
			//'#nnncc'でないので処理不能
			return false;
		}
	}
	//オブジェ配置
	return ParseObjLine(line, out);
}

bool BmsLoader::Load(const std::string& path, ScoreData* out){
	fs::ifstream ifs(path);
	if(!ifs) throw OpenError(std::string("\"") + path + "\" could not be opened.");
	{
		std::istreambuf_iterator<char> it(ifs);
		std::istreambuf_iterator<char> last;
		std::string file_str(it, last);
		ifs.clear();  // ここでclearしてEOFフラグを消す / clear EOF flag.
		ifs.seekg(0, ifs.beg);
		if(!StringToMD5(file_str, &out->info.md5)){
			throw ParseError("MD5 of the file could not be got.");
		}
	}
	std::string line;
	while(std::getline(ifs, line)){
		ParseLine(line.c_str(), out);
	}
	return true;
}

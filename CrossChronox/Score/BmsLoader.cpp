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

const int MAX_INDEX = 36 * 36; //ZZ(36)
const std::vector<ScoreInfo::judge_ms_type> rank_to_judge_ms = {
	{ 8, 24, 40 },
	{ 15, 30, 60 },
	{ 18, 40, 100 },
	{ 21, 60, 120 }
};

int BmsLoader::GetIndex(){
	const char* line = nowline;
	while(!std::isblank(*line)) ++line;
	std::string tmp = { line - 2, line - 1 };
	return std::stoi(tmp, nullptr, 36);
}

void trim_left(const char*& str){
	while(std::isblank(*str)) ++str;
}

const char* BmsLoader::GetArg(){
	const char* line = nowline;
	while(!std::isblank(*line)) ++line;
	trim_left(line);
	return line;
}

std::array<std::string,6> difficulty_str = {
	"UNDEFINED", //0
	"BEGINNER",  //1
	"NORMAL",    //2
	"HYPER",     //3
	"ANOTHER",   //4
	"INSANE"     //5
};


bool BmsLoader::TryParseObjLine(){
	//0123456
	//#nnncc:001122
	const char* comma_pos = nowline + 6;
	if(*comma_pos != ':'){
		//'#nnncc:'でないので処理不能
		return false;
	}
	for(int i = 1; i <= 5; ++i){
		if(!std::isdigit(nowline[i])){
			//'#nnncc:'でないので処理不能
			return false;
		}
	}
	if(!parse_nextline_flag){
		return false;
	}
	
	//TODO: implement
	return true;
}

bool BmsLoader::TryParseHeaderLine(){
	const char* header = nowline + 1;
	try{
		//Control Flow (Nested #RANDOM is not support.)
		if(boost::istarts_with(header, "IF")){
			parse_nextline_flag = (random_num == atoi(GetArg()));
		}
		else if(boost::istarts_with(header, "END")){
			parse_nextline_flag = true;
		}
		else if(boost::istarts_with(header, "RANDOM")){
			random_num = atoi(GetArg());
		}
		//Other Headers
		else if(parse_nextline_flag){
			//読み込みを高速化するため、定義が出てきやすいヘッダーを先に判定する
			//WAVやBPM,LNOBJは複数個出てきやすい
			if(boost::istarts_with(header, "WAV")){
				out->sound_channels[GetIndex()].name = GetArg();
			}
			else if(boost::istarts_with(header, "BPM")){
				//0123456
				//BPM 130
				//BPM01 130
				if(std::isblank(header[3])){
					out->info.init_bpm = atof(GetArg());
				}
				else{
					exbpm[GetIndex()] = atof(GetArg());
				}
			}
			else if(boost::istarts_with(header, "LNOBJ")){
				lnobj.emplace_back(std::stoi(GetArg(), nullptr, 36));
			}
			else if(boost::istarts_with(header, "TITLE")){
				out->info.title = GetArg();
			}
			else if(boost::istarts_with(header, "SUBTITLE")){
				out->info.subtitle = GetArg();
			}
			else if(boost::istarts_with(header, "ARTIST")){
				out->info.artist = GetArg();
			}
			else if(boost::istarts_with(header, "SUBARTIST")){
				out->info.subartists.emplace_back(GetArg());
			}
			else if(boost::istarts_with(header, "GENRE")){
				out->info.genre = GetArg();
			}
			else if(boost::istarts_with(header, "TOTAL")){
				out->info.total_type = TOTAL_ABSOLUTE;
				out->info.total = atof(GetArg());
				if(out->info.total < 20) throw BmsLoader::ParseError("TOTAL is not appropriate.");
			}
			else if(boost::istarts_with(header, "BACKBMP")){
				out->info.back_image = GetArg();
			}
			else if(boost::istarts_with(header, "STAGEFILE")){
				out->info.eyecatch_image = GetArg();
			}
			else if(boost::istarts_with(header, "BANNER")){
				out->info.banner_image = GetArg();
			}
			else if(boost::istarts_with(header, "PREVIEWMUSIC")){
				out->info.preview_music = GetArg();
			}
			else if(boost::istarts_with(header, "PLAYLEVEL")){
				out->info.level = atoi(GetArg());
			}
			else if(boost::istarts_with(header, "DIFFICULTY")){
				out->info.difficulty = atoi(GetArg());
				out->info.chart_name = difficulty_str.at(out->info.difficulty);
			}
			else if(boost::istarts_with(header, "RANK")){
				int i = boost::algorithm::clamp(atoi(GetArg()), 0, 3);
				out->info.judge_ms = rank_to_judge_ms.at(atoi(GetArg()));
			}
			else if(boost::istarts_with(header, "BASEBPM")){
				out->info.base_bpm = atof(GetArg());
			}
		}
	}
	catch(std::out_of_range& e){
		std::stringstream msg;
		msg << "Invalid arguments detected. LINE:";
		msg << line_num;
		msg << "\n";
		msg << e.what();
		throw ParseError(msg.str());
	}
}

bool BmsLoader::ParseLine(){
	trim_left(nowline);
	//コマンド行でないならreturn
	if(nowline[0] != '#'){
		if(boost::starts_with(nowline, "＃")) ++nowline;
		else return false;
	}
	
	if(TryParseObjLine() || TryParseHeaderLine()){
		//Succeed to parse
		return true;
	}
	return false;
}

bool BmsLoader::Init(ScoreData* out){
	*out = ScoreData();
	out->sound_channels.resize(MAX_INDEX);
	
	exbpm.clear();
	lnobj.clear();
	line_num = random_num = 0;
	parse_nextline_flag = true;
}

bool BmsLoader::Load(const std::string& path, ScoreData* out){
	this->out = out;
	Init(out);
	fs::ifstream ifs(path);
	if(!ifs) throw OpenError(std::string("\"") + path + "\" could not be opened.");
	{
		std::istreambuf_iterator<char> it(ifs);
		std::istreambuf_iterator<char> last;
		std::string file_str(it, last);
		ifs.clear();  // ここでclearしてEOFフラグを消す // clear EOF flag.
		ifs.seekg(0, ifs.beg);
		if(!StringToMD5(file_str, &out->info.md5)){
			throw ParseError("MD5 of the file could not be got.");
		}
	}
	std::string line;
	while(std::getline(ifs, line)){
		nowline = line.c_str();
		ParseLine();
		++line_num;
	}
	return true;
}






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

class BmsLoader : private boost::noncopyable{
	friend void LoadBms(const std::string& path, ScoreData* out) throw(LoadError, OpenError, ParseError);
	
	static const pulse_t BEAT_RESOLUTION = 240;
	
	struct BarInfo{
		double scale;
		pulse_t start_pulse;
		pulse_t length(){
			return scale * BEAT_RESOLUTION * 4;
		}
		BarInfo(): scale(1){}
		BarInfo(double d): scale(d){}
	};
	struct TmpNoteData{
		int bar;
		int channel;
		int index;
		
		bool lnend = false;
		
		pulse_t bar_pulse = 0;
		pulse_t global_pulse = 0;
		
		TmpNoteData(){}
		TmpNoteData(int bar, int channel, int index, int tmp_bar_linepulse, int tmp_bar_lineresolution): bar(bar), channel(channel), index(index), bar_pulse(4 * BEAT_RESOLUTION * tmp_bar_linepulse / tmp_bar_lineresolution){}
		
		bool operator < (const TmpNoteData& b) const{
			return global_pulse < b.global_pulse;
		}
	};
	std::array<BarInfo,1001> bar_info;
	int max_bar = 0;
	std::vector<std::unique_ptr<TmpNoteData>> tmp_notes;
	std::unordered_map<int, double> exbpm;
	std::unordered_map<int, int> stop;
	std::vector<int> lnobj;
	ScoreData* out = nullptr;
	const char* nowline = nullptr;
	int line_num = 0; //行番号
	std::bitset<30> channel_used_flag; //false filled
	
	int random_num = 0;
	bool parse_nextline_flag = true;
	
	std::string extention;
	std::function<int(int)> ChannelToX;
	
	const char* GetArg();
	int GetIndex();
	int GetIndex(const char* str, int base = 36);
	bool TryParseLine();
	bool TryParseHeaderLine();
	bool TryParseObjLine();
	void InitCommands();
	void Init(ScoreData* out);
	void SetSubtitle();
	void SetMode();
	void SetNotesAndEvents();
	void SetNoteTime();
	void SetBpm();
	void LoadWavs(const std::string& path);

	BmsLoader(){}
	void Load(const std::string& path, ScoreData* out);
};

const int MAX_INDEX = 36 * 36; //ZZ(36)
const std::vector<ScoreInfo::judge_ms_type> rank_to_judge_ms = {
	{ 8, 24, 40 },  //RANK 0 VERYHARD
	{ 15, 30, 60 }, //RANK 1 HARD
	{ 18, 40, 100 },//RANK 2 NORMAL
	{ 21, 60, 120 } //RANK 3 EASY
};

const int CHANNEL_BGM = 1;
const int CHANNEL_METER = 2;
const int CHANNEL_BPM = 3;
const int CHANNEL_BGABASE = 4;
const int CHANNEL_BGAPOOR = 6;
const int CHANNEL_BGALAYER = 7;
const int CHANNEL_EXBPM = 8;
const int CHANNEL_STOPS = 9;

const auto BmsChannelToX = [](int channel){
	if(61 <= channel) channel -= 40;
	switch(channel){
		case CHANNEL_BGM:
			return 0;
		case 11:
		case 12:
		case 13:
		case 14:
		case 15:
			return channel - 10; //1P KEY1-5
		case 16:
			return 8; //SC
			//case 17: //FREEZONE
		case 18:
		case 19:
			return channel - 12; //1P KEY6-7
		case 21:
		case 22:
		case 23:
		case 24:
		case 25:
			return channel - 12; //2P KEY1-5
		case 26:
			return 16; //SC
			//case 27: //FREEZONE
		case 28:
		case 29:
			return channel - 14; //2P KEY6-7
		default:
			return -1;
	}
};

const auto PmsChannelToX = [](int channel){
	if(61 <= channel) channel -= 40;
	switch(channel){
		case CHANNEL_BGM:
			return 0;
		case 11:
		case 12:
		case 13:
		case 14:
		case 15:
			return channel - 10; //KEY1-5
		case 22:
		case 23:
		case 24:
		case 25:
			return channel - 16; //KEY6-9
		default:
			return -1;
	}
};

const auto PmeChannelToX = [](int channel){
	if(61 <= channel) channel -= 40;
	switch(channel){
		case CHANNEL_BGM:
			return 0;
		case 11:
		case 12:
		case 13:
		case 14:
		case 15:
			return channel - 10; //KEY1-5
		case 18:
		case 19:
			return channel - 12; //KEY6-7
		case 16:
		case 17:
			return channel - 8;  //KEY8-9
		default:
			return -1;
	}
};

const int MAX_X = 20;

int BmsLoader::GetIndex(){
	const char* line = nowline;
	while(!std::isblank(*line)) ++line;
	std::string tmp = { line - 2, line - 1 };
	return std::stoi(tmp, nullptr, 36);
}

int BmsLoader::GetIndex(const char* str, int base){
	std::string tmp = { str[0], str[1] };
	return std::stoi(tmp, nullptr, base);
}

void trim_left(const char*& str){
	while(std::isblank(*str)) ++str;
}

void trim_right(char* str){
	char* blank_ptr = nullptr;
	for(; *str; ++str){
		if(std::isblank(*str)){
			if(blank_ptr == nullptr) blank_ptr = str;
		}
		else{
			blank_ptr = nullptr;
		}
	}
	if(blank_ptr) *blank_ptr = '\0';
}

const char* BmsLoader::GetArg(){
	const char* line = nowline;
	while(!std::isblank(*line)) ++line;
	trim_left(line);
	return line;
}

const std::vector<std::string> difficulty_str = {
	"UNDEFINED", //0
	"BEGINNER",  //1
	"NORMAL",    //2
	"HYPER",     //3
	"ANOTHER",   //4
	"INSANE"     //5
};

bool BmsLoader::TryParseObjLine(){
	//01234567
	//#nnncc:001122
	if(nowline[6] != ':'){
		//'#nnncc:'でないので処理不能
		return false;
	}
	for(int i = 1; i <= 5; ++i){
		if(!std::isalnum(nowline[i])){
			//'#nnncc:'でないので処理不能
			return false;
		}
	}
	if(!parse_nextline_flag){
		return false;
	}
	
	//it is obj line. start parsing.
	//parse '#nnncc:'
	std::string tmps = { nowline[1], nowline[2], nowline[3] };
	int bar = std::stoi(tmps, nullptr, 10);
	int channel = GetIndex(nowline + 4);
	max_bar = std::max(max_bar, bar);
	
	//parse maindata (ex)'112A33ZZ'
	const char* arg = nowline + 7;
	trim_left(arg);
	if(11 <= channel){
		if(channel <= 29) channel_used_flag[channel] = true;
		else if(channel <= 49) channel_used_flag[channel - 20] = true;
		else if(channel <= 69) channel_used_flag[channel - 40] = true;
	}
	if(channel == CHANNEL_METER){
		bar_info[bar].scale = std::atof(arg);
	}
	else{
		int len = 0;
		int base = (channel == CHANNEL_BPM ? 16 : 36);
		while(isalnum(arg[len])) ++len;
		int resolution = len / 2;
		for(int i = 0; i < resolution; ++i){
			int index = GetIndex(arg + i * 2, base);
			if(index){
				tmp_notes.emplace_back(new TmpNoteData(bar, channel, index, i, resolution));
			}
		}
	}
	
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
			int random_max = atoi(GetArg());
			std::uniform_int_distribution<> dist(1, random_max);
			random_num = dist(mt_rand);
			out->info.random_flag = true;
		}
		//Other Headers
		else if(parse_nextline_flag){
			//読み込みを高速化するため、定義が出てきやすいヘッダーを先に判定する
			//WAVやBPM,LNOBJ,STOPは複数個出てきやすい
			if(boost::istarts_with(header, "WAV")){
				//out->sound_channels[GetIndex()].name = GetArg();
				auto it = out->wavbufs.cbegin() + GetIndex();
                out->wavbufs.emplace(it, new WavBuffer(GetArg()));
			}
			else if(boost::istarts_with(header, "BPM")){
				//0123456
				//BPM 130
				//BPM01 130
				if(std::isblank(header[3])){
					int bpm = out->info.init_bpm = atof(GetArg());
					out->bpm_events.emplace_back(new BpmEvent(0, bpm));
				}
				else{
					exbpm[GetIndex()] = atof(GetArg());
				}
			}
			else if(boost::istarts_with(header, "LNOBJ")){
				lnobj.emplace_back(std::stoi(GetArg(), nullptr, 36));
			}
			else if(boost::istarts_with(header, "STOP")){
				stop[GetIndex()] = atoi(GetArg());
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
				if(out->info.total < 20) throw ParseError("TOTAL is not appropriate.");
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
				out->info.judge_ms = rank_to_judge_ms.at(i);
			}
			else if(boost::istarts_with(header, "BASEBPM")){
				out->info.base_bpm = atof(GetArg());
			}
		}
        else{
            //if header command was not found
            return 0;
        }
        //if command was found
        return 1;
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

bool BmsLoader::TryParseLine(){
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

void BmsLoader::SetSubtitle(){
	if(out->info.subtitle.empty()){
		//Find implicit subtitle.
		//(ex) Shadowgaze [ANOTHER]
		//"delim" is omitted from "delimiter"
		typedef std::pair<std::string,std::string> delim_type;
		static const std::vector<delim_type> delim_list = {
			{ "-", "-" },
			{ "〜", "〜" },
			{ "(", ")" },
			{ "[", "]" },
			{ "<", ">" },
			{ "\"", "\"" }
		};
		boost::trim_right(out->info.title);
		boost::string_ref title = out->info.title;
		for(delim_type delim : delim_list){
			if(boost::ends_with(title, delim.second)){
				auto pos_delim1 = title.rfind(delim.first);
				if(pos_delim1 != std::string::npos && pos_delim1 != 0){
					out->info.subtitle = title.substr(pos_delim1).to_string();
					out->info.title = title.substr(0, pos_delim1).to_string();
					boost::trim_right(out->info.title);
					break;
				}
			}
		} //End of for.
	}
}

void BmsLoader::SetMode(){
	int used_channel_max = 29;
	for(;;){
		if(channel_used_flag[used_channel_max] == false) break;
		if(used_channel_max == 11) break;
		--used_channel_max;
	}
	
	//bms, bme, bml
	if(boost::istarts_with(extention, "b")){
		ChannelToX = BmsChannelToX;
		if(28 <= used_channel_max){
			out->info.mode = MODE_BEAT_14K;
		}
		else if(21 <= used_channel_max){
			if(channel_used_flag[18] || channel_used_flag[19]){
				out->info.mode = MODE_BEAT_14K;
			}
			else out->info.mode = MODE_BEAT_10K;
		}
		else if(18 <= used_channel_max){
			out->info.mode = MODE_BEAT_7K;
		}
		else out->info.mode = MODE_BEAT_5K;
	}
	
	//pms, pme(pms BME-TYPE)
	else{
		if(24 <= used_channel_max){
			out->info.mode = MODE_POPN_9K;
			ChannelToX = PmsChannelToX;
		}
		else if(22 <= used_channel_max){
			if(channel_used_flag[11] || channel_used_flag[12]){
				out->info.mode = MODE_POPN_9K;
				ChannelToX = PmsChannelToX;
			}
			else{
				out->info.mode = MODE_POPN_5K;
				ChannelToX = PmsChannelToX;
			}
		}
		else{
			out->info.mode = MODE_POPN_9K;
			ChannelToX = PmeChannelToX;
		}
	}
}

void BmsLoader::SetNotesAndEvents(){
	//Set bar_info
	++max_bar;
	if(max_bar > 1000){
		throw ParseError("Too many bars. (max_bar > 1000)");
	}
	int total_pulse = 0;
	for(int i = 0; i <= max_bar; ++i){
		bar_info[i].start_pulse = total_pulse;
		total_pulse += bar_info[i].length();
		out->lines.emplace_back(total_pulse);
	}
	
	//Set bar_pulse and global_pulse to tmp_notes
	for(auto& note : tmp_notes){
		note->global_pulse = bar_info[note->bar].start_pulse + note->bar_pulse;
	}
    
	//Sort tmp_notes
    boost::sort(tmp_notes, ptr_less<TmpNoteData>());
	
	//Sort lnobj
	boost::sort(lnobj);
	
	size_t note_count = 0;
	
	//tmp_notes -> ScoreData
	bool ln_pushing[MAX_X] = {false};  //fill by 'false'
    std::array<Note*, MAX_X> last_note = {nullptr}; //last note of the lane
	for(const auto& tmp_note : tmp_notes){
		switch(tmp_note->channel){
			case CHANNEL_METER:
                assert(false);
				
			case CHANNEL_BGABASE:
			case CHANNEL_BGAPOOR:
			case CHANNEL_BGALAYER:
				break; //ignore //TODO: implement
				
			case CHANNEL_BPM:
				out->bpm_events.emplace_back(new BpmEvent(tmp_note->global_pulse, tmp_note->index));
				break;
				
			case CHANNEL_EXBPM:
				try{
					out->bpm_events.emplace_back(new BpmEvent(tmp_note->global_pulse, exbpm.at(tmp_note->index)));
				}
				catch(std::out_of_range&){
					//default of exBPM is 120
					out->bpm_events.emplace_back(new BpmEvent(tmp_note->global_pulse, 120));
				}
				break;
				
			case CHANNEL_STOPS:
				try{
					out->bpm_events.emplace_back(new StopEvent(tmp_note->global_pulse, 10 * stop.at(tmp_note->index)));
				}
				catch(std::out_of_range&){
					throw ParseError("Some errors are found. (#STOP)");
				}
				break;
				
			case CHANNEL_BGM:
			default: //Normal note, LN (invisible note is not implemented yet)
				bool ln_channel_flag = (51 <= tmp_note->channel && tmp_note->channel <= 69);
				int x = ChannelToX(tmp_note->channel);
				bool lnend_flag = false;
				
				if(ln_channel_flag){
					if(ln_pushing[x]) lnend_flag = true;
					ln_pushing[x] = !ln_pushing[x];
				}
				else if(boost::binary_search(lnobj,tmp_note->index)) lnend_flag = true;
				if(lnend_flag){
					last_note[x]->l = tmp_note->global_pulse - last_note[x]->y;
					break;
				}
				if(0 <= x){
					size_t num = 0;
					if(0 < x){
						num = note_count;
						++note_count;
					}
					out->notes.emplace_back(new Note(x, tmp_note->global_pulse, 0, num, out->wavbufs[tmp_note->index].get()));
					last_note[x] = out->notes.back().get();
				}
				break;
		}
	}
	//(last note count) + 1 is the number of notes
	//最後のnote_count+1がノーツの個数
	out->info.note_count = note_count + 1;
}

void BmsLoader::SetNoteTime(){
    BpmEvent* bpm_event = out->bpm_events.cbegin()->get();
	BpmEvent* end = out->bpm_events.cend()->get();
	BpmEvent* next_bpm_event = bpm_event + 1;
	const auto resolution = out->info.resolution;
    for(auto& note : out->notes){
		while(next_bpm_event != end && next_bpm_event->y < note->y){ // bpm_eventを後に処理
			++bpm_event;
			++next_bpm_event;
		}
		note->ms = bpm_event->NextEventMs(note->y, resolution);
	}
}

void BmsLoader::SetBpm(){
	double init = out->info.init_bpm;
	double max = init, min = init;
	
	std::unordered_map<int, double> bpm_length;
	BpmEvent init_bpm_event(0, init);
	const BpmEvent* last = &init_bpm_event;
	const BpmEvent* last_bpm_change = &init_bpm_event;
	
	for(auto& event : out->bpm_events){
		if(event->duration == 0){ //if event is BpmEvents
			event->duration = event->y - last_bpm_change->y;
			bpm_length[static_cast<int>(last->bpm)] += event->duration * last->bpm;
			max = std::max(max,event->bpm);
			min = std::min(min,event->bpm);
			last_bpm_change = event.get();
		}
		else{ //if event is StopEvents
			event->bpm = last_bpm_change->bpm;
		}
		last = event.get();
	}
	bpm_length[last->bpm] += (out->lines.back().y - last->y) * last->bpm;
	
	using pair_t = std::pair<int, double>;
	auto pred = [](const pair_t& a, const pair_t& b)->bool{
		return a.second < b.second;
	};
	auto it = std::max_element(bpm_length.begin(), bpm_length.end(), pred);
	out->info.base_bpm = it->first;
	out->info.max_bpm = max;
	out->info.min_bpm = min;
}

void BmsLoader::LoadWavs(const std::string& path){
	auto pos = path.find_last_of("/\\");
	std::string score_directory = path.substr(0, pos);
	for(auto& wavbuf : out->wavbufs){
		if(wavbuf){
			wavbuf->Load(score_directory);
		}
	}
}

void BmsLoader::Init(ScoreData* out){
	*out = ScoreData();
	out->wavbufs.resize(MAX_INDEX);
	
	exbpm.clear();
	lnobj.clear();
	line_num = random_num = 0;
	parse_nextline_flag = true;
}

void BmsLoader::Load(const std::string& path, ScoreData* out){
	this->out = out;
	Init(out);
	{
		fs::ifstream ifs(path);
		if(!ifs){
			throw OpenError(std::string("\"") + path + "\" could not be opened.");
		}
		
		//Get MD5 of the file
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
		
		//get extention from filename
		auto dot_pos = path.find_last_of('.');
		if(dot_pos != std::string::npos && dot_pos != path.length() - 1){
			extention = path.substr(dot_pos + 1);
		}
        else extention = "";
		
		//start parsing
		std::string line;
		while(std::getline(ifs, line)){
			//if CRLF and LF mixed, remove CR('\r')
			if(line.back() == '\r'){
				line.erase(line.end() - 1);
			}
			nowline = line.c_str();
			TryParseLine();
			++line_num;
		}
	}//file close
	
	SetSubtitle();
	SetMode();
	SetNotesAndEvents();
	SetNoteTime();
	SetBpm();
	LoadWavs(path);
}

void LoadBms(const std::string& path, ScoreData* out) throw(LoadError, OpenError, ParseError){
	BmsLoader loader;
	loader.Load(path, out);
}


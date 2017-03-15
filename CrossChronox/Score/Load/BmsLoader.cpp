//
//  BmsLoader.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 10/7/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#include "BmsLoader.hpp"
#include "Filesystem/MD5.hpp"

//BMS command memo (draft) (English)
//http://hitkey.nekokan.dyndns.info/cmds.htm

//BMS command memo (Japanese)
//http://hitkey.nekokan.dyndns.info/cmdsJP.htm

const int MAX_NOTE_CHANNEL = 30;

class BmsLoader : private boost::noncopyable{
	//friend void LoadBms(const std::string& path, ScoreData* out) throw(LoadError, OpenError, ParseError);
	
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
		TmpNoteData(int bar, int channel, int index, pulse_t tmp_bar_linepulse, pulse_t tmp_bar_lineresolution): bar(bar), channel(channel), index(index), bar_pulse(4 * BEAT_RESOLUTION * tmp_bar_linepulse / tmp_bar_lineresolution){}
		
		bool operator < (const TmpNoteData& b) const{
			return global_pulse < b.global_pulse;
		}
	};
	std::array<BarInfo,1001> bar_info;
	int max_bar = 0;
	std::vector<std::unique_ptr<TmpNoteData>> tmp_notes;
	std::unordered_map<int, double> exbpm;
	std::unordered_map<int, pulse_t> stop;
	std::vector<int> lnobj;
	ScoreData* out = nullptr;
	const char* nowline = nullptr;
	int line_num = 0; //行番号
	bool channel_used_flag[MAX_NOTE_CHANNEL] = {false}; //false filled
	
	int random_num = 0;
	bool parse_nextline_flag = true;
	
	std::string extention;
	std::function<lane_t(int)> ChannelToLane;
	
	bool load_header_only_flag = false;
	
	const char* text_encode = nullptr;
	
	bool IsTextAscii(const char* str){
		while(*str != '\0'){
			if(!isascii(*str)) return false;
		}
		return true;
	}
	
	enum TextEncode{
		UNKNOWN,
		UTF_8,
		SHIFT_JIS
	}
	encode = UNKNOWN;
	
	unsigned UTF8CharByte(unsigned char c){
		if(c <= 0x7f) return 1;
		if(0xc2 <= c){
			if(c <= 0xdf) return 2;
			if(c <= 0xef) return 3;
			if(c <= 0xf7) return 4;
			if(c <= 0xfb) return 5;
			if(c <= 0xfd) return 6;
		}
		return 0;
	}
	
	unsigned UTF8CharByte(char c){
		return UTF8CharByte(static_cast<unsigned char>(c));
	}
	
	TextEncode JudgeTextEncode(const char* str){
		//is UTF-8?
		for(const char* ptr = str; *ptr != '\0'; ++ptr){
			if(const unsigned n = UTF8CharByte(*ptr)){
				if(3 <= n) return UTF_8;
				if(n == 2){
					if(*(++ptr)){
						if(isascii(*ptr)) return SHIFT_JIS;
					}
					else return SHIFT_JIS;
				}
			}
			else return SHIFT_JIS;
		}
		return UNKNOWN;
	}
	
	std::string convert_encoding(const std::string& str, const char* fromcode, const char* tocode){
		iconv_t icd;
		size_t instr_len  = str.length();
		size_t outstr_len = instr_len*2;
		
		if (instr_len <= 0) return "";
		
		// allocate memory
		std::vector<char> instr_buf(instr_len+1), outstr_buf(outstr_len+1);
		char* instr = &instr_buf.front();
		char* outstr = &outstr_buf.front();
		strcpy(instr, str.c_str());
		icd = iconv_open(tocode, fromcode);
		if (icd == (iconv_t)-1) {
			return "Failed to open iconv (" + std::string(fromcode) + " to " + std::string(tocode) + ")";
		}
#if defined(_WIN64) || defined(_WIN32) //if Windows
		const
#endif
		char* src_pos = instr;
		char* dst_pos = outstr;
		if (iconv(icd, &src_pos, &instr_len, &dst_pos, &outstr_len) == -1) {
			// throw error message
			std::string errstr;
			int err = errno;
			if (err == E2BIG) {
				errstr = "There is not sufficient room at *outbuf";
			} else if (err == EILSEQ) {
				errstr = "An invalid multibyte sequence has been encountered in the input";
			} else if (err == EINVAL) {
				errstr = "An incomplete multibyte sequence has been encountered in the input";
			}
			iconv_close(icd);
			throw std::runtime_error(std::string("Failed to convert string (") + errstr + ")");
		}
		*dst_pos = '\0';
		iconv_close(icd);
		
		return outstr; //std::string(outstr);
	}
	
	//UTF8文字列からワイド文字列
	//ロケール依存
	void Widen(const char* src, std::wstring& dest){
		try{
			std::wstring_convert<std::codecvt_utf8<wchar_t>,wchar_t> cv;
			if(encode == UNKNOWN){
				encode = JudgeTextEncode(src);
			}
			if(encode != SHIFT_JIS){
				dest = cv.from_bytes(src);
				return;
			}
			dest = cv.from_bytes(convert_encoding(src, "Shift_JIS", "UTF-8"));
		}
		catch(std::range_error&){
			throw ParseError(std::string("could not read string. (") + src + ")");
		}
	}
	
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
	void SetBpm();
	void SetNoteTime();
	void SetTotal();
	void LoadWavs(const std::string& path);

public:
	BmsLoader(){}
	void Load(const std::string& path, ScoreData* out, bool load_header_only_flag = false);
};

const int MAX_INDEX = 36 * 36; //ZZ(36)
//const std::vector<ScoreInfo::judge_ms_type> rank_to_judge_ms = {
//	{ 8, 24, 40 },  //RANK 0 VERYHARD
//	{ 15, 30, 60 }, //RANK 1 HARD
//	{ 18, 40, 100 },//RANK 2 NORMAL
//	{ 21, 60, 120 } //RANK 3 EASY
//};

const int CHANNEL_BGM = 1;
const int CHANNEL_METER = 2;
const int CHANNEL_BPM = 3;
const int CHANNEL_BGABASE = 4;
const int CHANNEL_BGAPOOR = 6;
const int CHANNEL_BGALAYER = 7;
const int CHANNEL_EXBPM = 8;
const int CHANNEL_STOPS = 9;

const auto BmsChannelToLane = [](int channel){
	if(51 <= channel) channel -= 40;
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

const auto PmsChannelToLane = [](int channel){
	if(51 <= channel) channel -= 40;
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

const auto PmeChannelToLane = [](int channel){
	if(51 <= channel) channel -= 40;
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

int BmsLoader::GetIndex(){
	boost::string_ref line = nowline;
	//#WAV01 music.wav
	auto pos = line.find_first_of("vV");
	if(pos != std::string::npos){
		++pos;
	}
	else{
		//#BPM01 120, #BMP01 image.bmp
		pos = line.find_first_of("mMpP");
		if(pos != std::string::npos){
			pos += 2;
		}
	}
	return GetIndex(nowline + pos);
}

int BmsLoader::GetIndex(const char* str, int base){
	std::string tmp = { str[0], str[1] };
	return std::stoi(tmp, nullptr, base);
}

void trim_left(const char*& str){
	while(isblank(*str)) ++str;
}

void trim_right(char* str){
	char* blank_ptr = nullptr;
	for(; *str; ++str){
		if(isblank(*str)){
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
	while(!isblank(*line)) ++line;
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
		if(!isalnum(nowline[i])){
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
	int channel = GetIndex(nowline + 4, 10);
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
			if(!load_header_only_flag && boost::istarts_with(header, "WAV")){
				auto it = out->wavbufs.cbegin() + GetIndex();
				out->wavbufs.emplace(it, new WavBuffer(GetArg()));
			}
			else if(boost::istarts_with(header, "BPM")){
				//0123456
				//BPM 130
				//BPM01 130
				if(isblank(header[3])){
					double bpm = out->info.init_bpm = atof(GetArg());
					out->bpm_events.clear();
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
				Widen(GetArg(), out->info.title);
			}
			else if(boost::istarts_with(header, "SUBTITLE")){
				Widen(GetArg(), out->info.subtitle);
			}
			else if(boost::istarts_with(header, "ARTIST")){
				Widen(GetArg(), out->info.artist);
			}
			else if(boost::istarts_with(header, "SUBARTIST")){
				//out->info.subartists.emplace_back(sf::String(GetArg()).toWideString());
				out->info.subartists.emplace_back();
				Widen(GetArg(), out->info.subartists.back());
			}
			else if(boost::istarts_with(header, "GENRE")){
				Widen(GetArg(), out->info.genre);
			}
			else if(boost::istarts_with(header, "TOTAL")){
				out->info.total.SetValue(atof(GetArg()));
				if(out->info.total.GetValue() < 20) throw ParseError("TOTAL is not appropriate.");
			}
			else if(boost::istarts_with(header, "BACKBMP")){
				Widen(GetArg(), out->info.back_image);
			}
			else if(boost::istarts_with(header, "STAGEFILE")){
				Widen(GetArg(), out->info.eyecatch_image);
			}
			else if(boost::istarts_with(header, "BANNER")){
				Widen(GetArg(), out->info.banner_image);
			}
			else if(boost::istarts_with(header, "PREVIEWMUSIC")){
				Widen(GetArg(), out->info.preview_music);
			}
			else if(boost::istarts_with(header, "PLAYLEVEL")){
				out->info.level = atoi(GetArg());
			}
			else if(boost::istarts_with(header, "DIFFICULTY")){
				out->info.difficulty = atoi(GetArg());
				//out->info.chart_name = sf::String(difficulty_str.at(out->info.difficulty)).toWideString();
				Widen(difficulty_str.at(out->info.difficulty).c_str(), out->info.chart_name);
			}
			else if(boost::istarts_with(header, "RANK")){
				int i = boost::algorithm::clamp(atoi(GetArg()), 0, 3);
				out->info.judge_rank.SetValue(i);
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
		if(boost::starts_with(nowline, "＃")){
			++nowline;
		}
		else{
			return false;
		}
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
		typedef std::pair<wchar_t, wchar_t> delim_type;
		static const std::vector<delim_type> delim_list = {
			{ L'-', L'-' },
			{ L'〜', L'〜' },
			{ L'(', L')' },
			{ L'[', L']' },
			{ L'<', L'>' },
			{ L'\"', L'\"' }
		};
		boost::trim_right(out->info.title);
		boost::wstring_ref title = out->info.title;
		for(delim_type delim : delim_list){
			if(title.back() == delim.second){
				auto pos_delim1 = title.substr(0, title.length() - 1).find_last_of(delim.first);
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
	int used_channel_max = MAX_NOTE_CHANNEL - 1;
	for(;;){
		if(channel_used_flag[used_channel_max]) break;
		if(used_channel_max == 11) break;
		--used_channel_max;
	}
	
	//bms, bme, bml
	if(boost::istarts_with(extention, "b")){
		ChannelToLane = BmsChannelToLane;
		if(28 <= used_channel_max){
			out->info.mode = BEAT_14K;
		}
		else if(21 <= used_channel_max){
			if(channel_used_flag[18] || channel_used_flag[19]){
				out->info.mode = BEAT_14K;
			}
			else out->info.mode = BEAT_10K;
		}
		else if(18 <= used_channel_max){
			out->info.mode = BEAT_7K;
		}
		else out->info.mode = BEAT_5K;
	}
	
	//pms, pme(pms BME-TYPE)
	else{
		if(24 <= used_channel_max){
			out->info.mode = POPN_9K;
			ChannelToLane = PmsChannelToLane;
		}
		else if(22 <= used_channel_max){
			if(channel_used_flag[11] || channel_used_flag[12]){
				out->info.mode = POPN_9K;
				ChannelToLane = PmsChannelToLane;
			}
			else{
				out->info.mode = POPN_5K;
				ChannelToLane = PmsChannelToLane;
			}
		}
		else{
			out->info.mode = POPN_9K;
			ChannelToLane = PmeChannelToLane;
		}
	}
}

void BmsLoader::SetNotesAndEvents(){
	//Set bar_info
	++max_bar;
	if(max_bar > 1000){
		throw ParseError("Too many bars. (max_bar > 1000)");
	}
	pulse_t total_pulse = 0;
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
	bool ln_pushing[MAX_LANE] = {false};  //fill by 'false'
    std::array<Note*, MAX_LANE> last_note = {{nullptr}}; //last note of the lane
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
				lane_t lane = ChannelToLane(tmp_note->channel);
				bool lnend_flag = false;
				
				if(ln_channel_flag){
					if(ln_pushing[lane]) lnend_flag = true;
					ln_pushing[lane] = !ln_pushing[lane];
				}
				else if(boost::binary_search(lnobj,tmp_note->index)) lnend_flag = true;
				if(lnend_flag){
					last_note[lane]->len = tmp_note->global_pulse - last_note[lane]->pulse;
					break;
				}
				if(0 <= lane){
					size_t num = 0;
					if(0 < lane){
						num = ++note_count;
					}
					out->notes.emplace_back(new Note(lane, tmp_note->global_pulse, 0, num, out->wavbufs[tmp_note->index].get()));
					last_note[lane] = out->notes.back().get();
				}
				break;
		}
	}
	//(last note count) is the number of notes
	//最後のnote_countがノーツの個数
	out->info.note_count = note_count;
}

void BmsLoader::SetBpm(){
	const double init = out->info.init_bpm;
	double max = init, min = init;
	
	std::unordered_map<int, double> bpm_length;
	//BpmEvent init_bpm_event(0, init);
	BpmEvent* last = nullptr;
	BpmEvent* last_bpm_change = out->bpm_events.begin()->get();
	
	for(auto& event : out->bpm_events){
		if(last){
			if(event->duration == 0){ //if event is BpmEvents
				last->duration = event->pulse - last_bpm_change->pulse;
				bpm_length[last->bpm + 0.5] += last->duration * last->bpm;
				max = std::max(max, event->bpm);
				min = std::min(min, event->bpm);
				last_bpm_change = event.get();
			}
			else{ //if event is StopEvents
				event->bpm = last_bpm_change->bpm;
			}
			event->ms = last->NextEventMs(event->pulse, out->info.resolution);
		}
		last = event.get();
	}
	//set end_pulse
	if(!tmp_notes.empty()){
		out->info.end_pulse = tmp_notes.back()->global_pulse;
	}
	bpm_length[last->bpm + 0.5] += (out->info.end_pulse - last->pulse) * last->bpm;
	
	using pair_t = std::pair<int, double>;
	auto pred = [](const pair_t& a, const pair_t& b)->bool{
		return a.second < b.second;
	};
	auto it = std::max_element(bpm_length.begin(), bpm_length.end(), pred);
	out->info.base_bpm = it->first;
	out->info.max_bpm = max;
	out->info.min_bpm = min;
}

void BmsLoader::SetNoteTime(){
	auto bpm_event_it = out->bpm_events.cbegin();
	auto back_it = --out->bpm_events.cend();
	
	const auto resolution = out->info.resolution;
    for(auto& note : out->notes){
		while(bpm_event_it != back_it && (*bpm_event_it)->pulse + (*bpm_event_it)->duration < note->pulse){ // bpm_eventを後に処理
			++bpm_event_it;
		}
		note->ms = (*bpm_event_it)->NextEventMs(note->pulse, resolution);
	}
	
	out->info.end_ms = out->bpm_events.back()->NextEventMs(out->info.end_pulse, out->info.resolution);
}

void BmsLoader::SetTotal(){
	auto& total = out->info.total;
	if(total.GetValue() < 20){
		auto notes = out->info.note_count;
		total.SetValue(7.605 * notes / (0.01 * notes + 6.5));
	}
}

void BmsLoader::LoadWavs(const std::string& path){
	auto pos = path.find_last_of("/\\");
	std::string score_directory = path.substr(0, pos + 1);
	for(auto& wavbuf : out->wavbufs){
		if(wavbuf){
			wavbuf->Load(score_directory);
		}
	}
}

void BmsLoader::Init(ScoreData* out){
	out->Init();
	out->wavbufs.resize(MAX_INDEX);
	
	exbpm.clear();
	lnobj.clear();
	line_num = random_num = 0;
	parse_nextline_flag = true;
}

void BmsLoader::Load(const std::string& path, ScoreData* out, bool load_header_only_flag){
	this->out = out;
	this->load_header_only_flag = load_header_only_flag;
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
		
		//set path to ScoreInfo
		out->info.path = path;
		
		//get extention from filename
		auto dot_pos = path.find_last_of('.');
		if(dot_pos != std::string::npos && dot_pos != path.length() - 1){
			extention = path.substr(dot_pos + 1);
		}
        else extention = "";
		
		//rank of bms is always absolute and NORMAL is default. (same to LR2)
		out->info.judge_rank = JudgeRank(RANK_ABSOLUTE, RANK_NORMAL);
		
		//total of bms is always absolute.
		out->info.total = Total(TOTAL_ABSOLUTE, 0);
		
		//default BPM130
		out->bpm_events.emplace_back(new BpmEvent(0, 130.0));
		
		//start parsing
		std::string line;
		while(std::getline(ifs, line)){
			//if CRLF and LF mixed, we must remove CR('\r')
			if(!line.empty() && line.back() == '\r'){
				line.pop_back(); //.erase(line.end() - 1);
			}
			nowline = line.c_str();
			TryParseLine();
			++line_num;
		}
	}//file close
	
	SetSubtitle();
	SetMode();
	SetNotesAndEvents();
	SetBpm();
	SetNoteTime();
	if(!load_header_only_flag) LoadWavs(path);
}

void LoadBms(const std::string& path, ScoreData* out, bool load_header_only_flag) throw(LoadError, OpenError, ParseError){
	BmsLoader loader;
	loader.Load(path, out, load_header_only_flag);
}

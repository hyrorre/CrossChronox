#include "ScoreInfo.hpp"

static const std::vector<std::string> mode_str = {
    "beat-5k",
    "beat-7k",
    "beat-10k",
    "beat-14k",
    "popn-5k",
    "popn-9k"};

const std::string& GetModeString(Mode mode) {
    return mode_str[static_cast<size_t>(mode)];
}

std::string ScoreInfo::GetInfoStr() const {
    std::stringstream ss;
#define SS(x) ss << #x ": " << x << '\n'
    SS(title);
    SS(subtitle);
    SS(artist);
    SS(genre);
    ss << "mode"
          ": "
       << GetModeString(mode).c_str() << '\n';
    SS(chart_name);
    SS(difficulty);
    SS(level);
    SS(note_count);
    SS(init_bpm);
    SS(base_bpm);
    SS(max_bpm);
    SS(min_bpm);
    ss << std::endl;
    return ss.str();
}

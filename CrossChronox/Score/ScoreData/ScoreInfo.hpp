#pragma once

#include "pch.hpp"
#include "Filesystem/ScoreInfoBase.hpp"
#include "JudgeRank.hpp"
#include "System/TimeManager.hpp"
#include "Total.hpp"

using pulse_t = unsigned long;

inline const std::wstring GetModeString(Mode mode) {
    const std::vector<std::wstring> mode_str = {
        L"beat-5k",
        L"beat-7k",
        L"beat-10k",
        L"beat-14k",
        L"popn-5k",
        L"popn-9k"};
    return mode_str[static_cast<size_t>(mode)];
}

enum LnType {
    LN_NORMAL,
    LN_CN,
    LN_HCN
};

struct ScoreInfo : public ScoreInfoBase {
    std::wstring title;                   // self-explanatory
    std::wstring subtitle;                // self-explanatory
    std::wstring artist;                  // self-explanatory
    std::vector<std::wstring> subartists; // ["key:value"]
    std::wstring genre;                   // self-explanatory
    // std::string   mode_hint = "beat-7k"; // layout hints, e.g. "beat-7k", "popn-5k", "generic-nkeys"
    Mode mode;
    LnType ln_type;
    std::wstring chart_name; // e.g. "HYPER", "FOUR DIMENSIONS"
    int difficulty = 0;
    int level = 0;               // self-explanatory
    double init_bpm = 130;       // self-explanatory
    JudgeRank judge_rank;        //
    Total total;                 // lifebar gain
    std::wstring back_image;     // background image filename
    std::wstring eyecatch_image; // eyecatch image filename
    std::wstring banner_image;   // banner image filename
    std::wstring preview_music;  // preview music filename
    pulse_t resolution = 240;    // pulses per quarter note
    pulse_t end_pulse = 0;
    ms_type end_ms = 0;

    double max_bpm;        // calc from the data
    double min_bpm;        // calc from the data
    double base_bpm = 0;   // calc from the data
    size_t note_count = 0; // calc from the data
    std::string md5;       // use to identify score in level table and IR
    // int peek_vol;                        // use it if replaygain is implemented
    bool random_flag = false; // if #RANDOM is used, it should not be registered IR

    std::wstring GetTitleSubtitle() const {
        std::wstringstream ss;
        ss << level << ' ' << title << ' ' << subtitle;
        return ss.str();
    }

    std::wstring GetInfoStr() const {
        std::wstringstream ss;
        ss << L"title" << title;
        ss << L"subtitle" << subtitle;
        ss << L"artist" << artist;
        ss << L"genre" << genre;
        ss << L"mode"
              L": "
           << GetModeString(mode) << L'\n';
        ss << L"chart_name" << chart_name;
        ss << L"difficulty" << difficulty;
        ss << L"level" << level;
        ss << L"note_count" << note_count;
        ss << L"init_bpm" << init_bpm;
        ss << L"base_bpm" << base_bpm;
        ss << L"max_bpm" << max_bpm;
        ss << L"min_bpm" << min_bpm;
        ss << std::endl;
        return ss.str();
    }

    ScoreInfo() {}
    ScoreInfo(std::string path) : ScoreInfoBase(path) {}

    template <class Context>
    constexpr static void serde(Context& context, ScoreInfo& value) {
        serde::serde_struct(context, value)
            .field(&ScoreInfo::title, "title")
            .field(&ScoreInfo::subtitle, "subtitle")
            .field(&ScoreInfo::artist, "artist")
            .field(&ScoreInfo::subartists, "subartists")
            .field(&ScoreInfo::genre, "genre")
            .field(&ScoreInfo::mode, "mode")
            .field(&ScoreInfo::chart_name, "chart_name")
            .field(&ScoreInfo::difficulty, "difficulty")
            .field(&ScoreInfo::level, "level")
            .field(&ScoreInfo::init_bpm, "init_bpm")
            .field(&ScoreInfo::judge_rank, "judge_rank")
            .field(&ScoreInfo::total, "total")
            .field(&ScoreInfo::back_image, "back_image")
            .field(&ScoreInfo::eyecatch_image, "eyecatch_image")
            .field(&ScoreInfo::banner_image, "banner_image")
            .field(&ScoreInfo::preview_music, "preview_music")
            .field(&ScoreInfo::resolution, "resolution")
            .field(&ScoreInfo::end_pulse, "end_pulse")
            .field(&ScoreInfo::end_ms, "end_ms")
            .field(&ScoreInfo::max_bpm, "max_bpm")
            .field(&ScoreInfo::min_bpm, "min_bpm")
            .field(&ScoreInfo::base_bpm, "base_bpm")
            .field(&ScoreInfo::note_count, "note_count")
            .field(&ScoreInfo::md5, "md5")
            .field(&ScoreInfo::random_flag, "random_flag")
            .field(&ScoreInfo::ln_type, "ln_type");
    }
};

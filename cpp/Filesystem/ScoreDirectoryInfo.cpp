#include "ScoreDirectoryInfo.hpp"
#include "Path.hpp"
#include "ScoreDirectoryLoader.hpp"

std::string score_info_cache_path;

void SetScoreInfoCachePath() {
    static bool inited = false;
    if (!inited) {
        inited = true;
        fs::path database = GetAppdataPath() / "Database";
        if (!fs::exists(database))
            fs::create_directories(database);
        score_info_cache_path = (database / "Song.toml").string();
    }
}

void ScoreDirectoryInfo::LoadScoreDirectory() {
    children.clear();
    ScoreDirectoryLoader().Load(GetAppdataPath() / "Songs", this);
}

void ScoreDirectoryInfo::SaveScoreDirectoryCache() const {
    // SetScoreInfoCachePath();
    // std::ofstream ofs(score_info_cache_path);
    // auto str = serde::serialize<serde::toml_v>(*this);
    // ofs << str;
}

bool ScoreDirectoryInfo::TryLoadScoreDirectoryCache() {
    try {
        SetScoreInfoCachePath();
        children.clear();

        // auto str = serde::parse_file<serde::toml_v>(score_info_cache_path);
        // *this = serde::deserialize<ScoreDirectoryInfo>(str);
        // return true;
        return false;
    } catch (std::exception& e) {
        Clear();
        return false;
    }
}

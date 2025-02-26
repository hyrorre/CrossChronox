﻿#pragma once

#include "pch.hpp"
#include "Score/ScoreData/ScoreData.hpp"
#include "ScoreDirectoryInfo.hpp"
#include "Score/Load/BmsLoader.hpp"

class ScoreDirectoryLoader {
    ScoreData tmp_data;

    const fs::directory_iterator end;

    const std::vector<std::string> bms_extentions = {
        "bms",
        "bme",
        "bml",
        "pms"
#if !defined(_WIN64) && !defined(_WIN32) // if not Windows
        ,
        "Bms",
        "BMS",
        "Bme",
        "BME",
        "Bml",
        "BML",
        "Pms",
        "PMS"
#endif
    };

    const std::vector<std::string> bmson_extentions = {
        "bmson"
#if !defined(_WIN64) && !defined(_WIN32) // if not Windows
        ,
        "Bmson",
        "BMSON"
#endif
    };

    bool IsScoreFolder(const fs::path& path) {
        for (fs::directory_iterator it(path); it != end; ++it) {
            std::string file_extention = it->path().extension().string();
            if (!file_extention.empty() && file_extention[0] == '.') {
                file_extention = file_extention.substr(1);
            }
            for (const auto& extention : bms_extentions) {
                if (file_extention == extention) {
                    return true;
                }
            }
            for (const auto& extention : bmson_extentions) {
                if (file_extention == extention) {
                    return true;
                }
            }
        }
        return false;
    }

    void LoadScores(const fs::path& path, ScoreDirectoryInfo* out) {
        for (fs::directory_iterator it(path); it != end; ++it) {
            if (fs::is_directory(*it))
                continue;
            std::string file_extention = it->path().extension().string();
            if (!file_extention.empty() && file_extention.front() == '.')
                file_extention = file_extention.substr(1);
            for (const auto& extention : bms_extentions) {
                if (file_extention == extention) {
                    LoadBms(it->path().string(), &tmp_data, true);
                    out->children.emplace_back(new ScoreInfo(tmp_data.info));
                }
            }
        }
    }

  public:
    void Load(const fs::path& path, ScoreDirectoryInfo* out) {
        out->children.clear();
        for (fs::directory_iterator it(path); it != end; ++it) {
            if (fs::is_directory(*it)) {
                if (IsScoreFolder(it->path())) {
                    LoadScores(it->path(), out);
                } else {
                    auto* child = new ScoreDirectoryInfo(it->path().string());
                    out->children.emplace_back(child);
                    child->parent = out;
                    child->title = it->path().filename().wstring();
                    Load(it->path(), child);
                }
            }
        }
    }
};

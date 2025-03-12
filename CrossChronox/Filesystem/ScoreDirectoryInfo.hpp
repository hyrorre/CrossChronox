#pragma once

#include "pch.hpp"
#include "ScoreInfoBase.hpp"

class ScoreDirectoryLoader;

class ScoreDirectoryInfo : public ScoreInfoBase {
    friend ScoreDirectoryLoader;

    ScoreDirectoryInfo* parent = nullptr;
    std::vector<std::unique_ptr<ScoreInfoBase>> children;
    int cursor = 0;
    std::string title;

  public:
    void DownMusic() {
        ++cursor;
    }
    void UpMusic() {
        --cursor;
    }
    std::string GetTitleSubtitle() const {
        return title;
    }
    ScoreDirectoryInfo* GetParent() {
        return parent;
    }
    void SetParent(ScoreDirectoryInfo* parent) {
        this->parent = parent;
    }
    const ScoreInfoBase* At(int index) const {
        int true_index = (cursor + index);
        for (; true_index < 0; true_index += children.size() * 10)
            ;
        true_index %= children.size();
        return children[true_index].get();
    }
    ScoreInfoBase* At(int index) {
        return const_cast<ScoreInfoBase*>(static_cast<const ScoreDirectoryInfo*>(this)->At(index));
    }
    bool Empty() {
        return children.empty();
    }
    void Clear() {
        children.clear();
    }

    std::string GetInfoStr() const {
        return "";
    }

    void LoadScoreDirectory();
    void SaveScoreDirectoryCache() const;
    void LoadScoreDirectorySaveCache() {
        LoadScoreDirectory();
        SaveScoreDirectoryCache();
    }
    bool TryLoadScoreDirectoryCache();

    ScoreDirectoryInfo() {}
    ScoreDirectoryInfo(std::string path) : ScoreInfoBase(path) {}

    // template <class Context>
    // constexpr static void serde(Context& context, ScoreDirectoryInfo& value) {
    //     serde::serde_struct(context, value)
    //         .field(&ScoreDirectoryInfo::title, "title")
    //         .field(&ScoreDirectoryInfo::children, "children");
    // }
};

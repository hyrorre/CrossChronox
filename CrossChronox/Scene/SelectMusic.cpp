#include "SelectMusic.hpp"
#include "Application.hpp"
#include "PlayScore.hpp"
#include "System/InputManager.hpp"

SelectMusic scene_select_music;

SelectMusic* scene_select_music_ptr = &scene_select_music;

// sf::Text text_songlist(font_default);
// sf::Text text_info(font_default);

Scene* SelectMusic::Update() {
    if (InputManager::GetKeyFuncState("Option").now > 0) {
        // TODO: 複数プレイヤーおよびDPに対応
        int player = 0;
        PlayOption& option = players[player].GetVariableAccount().info.GetVariablePlayOption();
        Side side = LEFT;
        if (InputManager::GetKeyFuncState("IncrPlacement").now == 1) {
            option.AddPlacement(side, 1);
        }
        if (InputManager::GetKeyFuncState("DecrPlacement").now == 1) {
            option.AddPlacement(side, -1);
        }
        if (InputManager::GetKeyFuncState("IncrGaugeType").now == 1) {
            option.AddGaugeType(1);
        }
        if (InputManager::GetKeyFuncState("DecrGaugeType").now == 1) {
            option.AddGaugeType(-1);
        }
        if (InputManager::GetKeyFuncState("IncrAssistType").now == 1) {
            option.AddAssistType(1);
        }
        if (InputManager::GetKeyFuncState("IncrFlip").now == 1) {
            option.SetFlip(!option.GetFlip());
        }
        if (InputManager::GetKeyFuncState("IncrDisplayArea").now == 1) {
            option.AddDisplayArea(1);
        }
    } else {
        if (InputManager::GetKeyFuncState("UpMusic").now == 1) {
            now_directory->UpMusic();
            // text_info.setString(now_directory->At(0)->GetInfoStr());
        }
        if (InputManager::GetKeyFuncState("DownMusic").now == 1) {
            now_directory->DownMusic();
            // text_info.setString(now_directory->At(0)->GetInfoStr());
        }
        if (InputManager::GetKeyFuncState("CloseFolder").now == 1) {
            if (now_directory->GetParent()) {
                now_directory = now_directory->GetParent();
                // text_info.setString("");
            }
        }
        if (InputManager::GetKeyFuncState("DecideMusic").now == 1) {
            ScoreInfoBase* tmp_info = now_directory->At(0);
            if (typeid(*tmp_info) == typeid(ScoreDirectoryInfo)) {
                ScoreDirectoryInfo* tmp_directory = static_cast<ScoreDirectoryInfo*>(tmp_info);
                if (!tmp_directory->Empty()) {
                    now_directory = tmp_directory;
                    // text_info.setString(now_directory->At(0)->GetInfoStr());
                }
            } else {
                Application::SetScoreFilePath(tmp_info->path);
                return scene_play_score_ptr;
            }
        }
        if (InputManager::GetKeyFuncState("ReloadFolder").now == 1) {
            root.LoadScoreDirectory();
            root.SaveScoreDirectoryCache();
        }
    }
    return scene_select_music_ptr;
}

TTF_Font* font = nullptr;

void SelectMusic::Init() {
    if (!inited) {
        inited = true;
        if (!root.TryLoadScoreDirectoryCache()) {
            root.LoadScoreDirectory();
            root.SaveScoreDirectoryCache();
        }

        if (!font)
            font = TTF_OpenFont((GetAppdataPath() / "Fonts/kazesawa/Kazesawa-Regular.ttf").string().c_str(), 24);

        // text_songlist.setPosition({1000.0f, 50.0f});
        // text_songlist.setScale({1.5f, 1.5f});

        // text_info.setCharacterSize(40);
    }
}

std::string str_songlist;

void SelectMusic::Draw(SDL_Renderer* render_target) const {
    if (InputManager::GetKeyFuncState("Option").now > 0) {
        str_songlist.clear();
        int player = 0;
        PlayOption& option = players[player].GetVariableAccount().info.GetVariablePlayOption();
        str_songlist +=
            std::string("PLACE : ") + placement_str[option.GetPlacement(LEFT)] + '\n' +
            "GAUGE : " + gauge_type_str[option.GetGaugeType()] + '\n' +
            "ASSYST : " + assist_str[option.GetAssistType()] + '\n' +
            "DISPLAY_AREA : " + display_area_str[option.GetDisplayArea()] + '\n' +
            "FLIP : " + (option.GetFlip() ? "ON" : "OFF");
    } else {
        str_songlist.clear();
        for (int i = -3; i < 5; ++i) {
            if (i == 0)
                str_songlist += '→';
            str_songlist += now_directory->At(i)->GetTitleSubtitle();
            str_songlist += "\n\n";
        }
    }
    SDL_Color textColor = {255, 255, 255, 255};
    SDL_Surface* textSurface = TTF_RenderText_Solid_Wrapped(font, str_songlist.c_str(), 0, textColor, 0);
    SDL_Texture* textTexture = SDL_CreateTextureFromSurface(render_target, textSurface);
    SDL_FRect textRect = {100, 50, textSurface->w, textSurface->h};
    SDL_RenderTexture(render_target, textTexture, nullptr, &textRect);
    SDL_DestroySurface(textSurface);
    SDL_DestroyTexture(textTexture);
    // text_songlist.setString(str_songlist);
    // render_target.draw(text_songlist);
    // render_target.draw(text_info);
}

#include "PlayScore.hpp"
#include "Filesystem/Path.hpp"
#include "SelectMusic.hpp"
#include "System/InputManager.hpp"

PlayScore scene_play_score;

PlayScore* scene_play_score_ptr = &scene_play_score;

void PlayScore::Deinit() {
    // これが無いと恐らくリソースの解放順の問題で強制終了する
    for (auto& player : players) {
        player.Clear();
    }
}

Scene* PlayScore::Update() {
    if (InputManager::GetKeyState("Esc").now == 1) {
        return scene_select_music_ptr;
    }
    int continue_flag = 0;
    for (auto& player : players) {
        continue_flag += player.Update();
    }
    if (continue_flag)
        return scene_play_score_ptr;
    else
        return scene_select_music_ptr;
}

unsigned scr_w = 89;
unsigned white_w = 51;
unsigned black_w = 39;

unsigned note_h = 10;

unsigned space = 3;

float scr_x = 76;

unsigned judgeline_w = scr_w + white_w * 4 + black_w * 3 + space * 7;

float judgeline_y = 715;

float judge_combo_x = scr_x + 50;

float judge_combo_y = judgeline_y - 180;

TTF_Font* font_playinfo = nullptr;
TTF_Font* font_judge = nullptr;

SDL_Texture* background = nullptr;
SDL_Texture* white = nullptr;
SDL_Texture* black = nullptr;
SDL_Texture* scr = nullptr;

SDL_Color color_judge_combo[] = {
    {0, 0, 255, 255}, // PGREAT
    {0, 0, 255, 255}, // GREAT
    {0, 0, 255, 255}, // GOOD
    {0, 0, 255, 255}, // BAD
    {0, 0, 255, 255}, // POOR
};

std::string str_judge_combo[] = {
    "GREAT ", // PGREAT
    "GREAT ", // GREAT
    "GOOD ",  // GOOD
    "BAD ",   // BAD
    "POOR ",  // POOR
};

void PlayScore::Init(SDL_Renderer* renderer) {
    if (!font_playinfo) {
        font_playinfo = TTF_OpenFont((GetAppdataPath() / "Fonts/kazesawa/Kazesawa-Regular.ttf").string().c_str(), 19);
        font_judge = TTF_OpenFont((GetAppdataPath() / "Fonts/kazesawa/Kazesawa-Regular.ttf").string().c_str(), 68);
        // TTF_SetFontOutline(font_judge, 1);

        background = IMG_LoadTexture(renderer, (GetAppdataPath() / "play.bmp").string().c_str());

        const SDL_PixelFormatDetails* format_details = SDL_GetPixelFormatDetails(SDL_PIXELFORMAT_RGBA8888);

        SDL_Surface* white_surface = SDL_CreateSurface(white_w, note_h, SDL_PIXELFORMAT_RGBA8888);
        SDL_FillSurfaceRect(white_surface, nullptr, SDL_MapRGBA(format_details, nullptr, 255, 255, 255, 255));
        white = SDL_CreateTextureFromSurface(renderer, white_surface);
        SDL_DestroySurface(white_surface);

        SDL_Surface* black_surface = SDL_CreateSurface(black_w, note_h, SDL_PIXELFORMAT_RGBA8888);
        SDL_FillSurfaceRect(black_surface, nullptr, SDL_MapRGBA(format_details, nullptr, 0, 0, 255, 255));
        black = SDL_CreateTextureFromSurface(renderer, black_surface);
        SDL_DestroySurface(black_surface);

        SDL_Surface* scr_surface = SDL_CreateSurface(scr_w, note_h, SDL_PIXELFORMAT_RGBA8888);
        SDL_FillSurfaceRect(scr_surface, nullptr, SDL_MapRGBA(format_details, nullptr, 255, 0, 0, 255));
        scr = SDL_CreateTextureFromSurface(renderer, scr_surface);
        SDL_DestroySurface(scr_surface);
    }

    for (auto& player : players) {
        player.Init();
    }
    ScorePlayer::Start();
}

float GetNoteX(lane_t lane) {
    switch (lane) {
    case 8:
        return scr_x;
    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
    case 7:
        return scr_x + scr_w + int(lane / 2) * white_w + int((lane - 1) / 2) * black_w + lane * space;
    default:
        return 0;
    }
}

SDL_Texture* GetNoteTexture(lane_t lane) {
    switch (lane) {
    case 8:
        return scr;
    case 1:
    case 3:
    case 5:
    case 7:
        return white;
    case 2:
    case 4:
    case 6:
        return black;
    default:
        return nullptr;
    }
}

float global_scroll = .7f * 480;

#define SS(x) ss << #x ": " << x << '\n'

void PlayScore::Draw(SDL_Renderer* renderer) const {
    std::stringstream ss;

    SDL_RenderTexture(renderer, background, nullptr, nullptr);
    for (auto& player : players) {
        ms_type play_ms = player.GetPlayMs();
        ms_type last_play_ms = play_ms - delta_ms;
        pulse_t now_pulse = player.GetScore().MsToPulse(play_ms);
        pulse_t last_pulse = player.GetScore().MsToPulse(last_play_ms);

        SS(play_ms);
        SS(now_pulse);

        const ScoreData& score = player.GetScore();
        ss << score.info.GetInfoStr();
        ss << player.GetResult().GetResultStr() << std::endl;
        for (const auto& note : score.notes) {
            float lnstart_y = judgeline_y - static_cast<int>((note->pulse - now_pulse) * global_scroll / score.info.resolution);
            float note_y = judgeline_y - static_cast<int>((note->pulse + note->len - now_pulse) * global_scroll / score.info.resolution);
            if (judgeline_y + note_h < note_y)
                continue;
            if (lnstart_y + note_h < 0)
                break;
            SDL_Texture* texture = GetNoteTexture(note->lane);
            if (texture) {
                if (note->len == 0) { // if note is not LN
                    SDL_FRect frect = {GetNoteX(note->lane), note_y, (float)texture->w, (float)note_h};
                    SDL_RenderTexture(renderer, texture, nullptr, &frect);
                } else { // if note is LN
                    if (note->pulse < now_pulse) {
                        lnstart_y = judgeline_y;
                    }

                    float lnend_y = note_y;
                    SDL_FRect frect = {GetNoteX(note->lane), note_y, (float)texture->w, lnstart_y - lnend_y + note_h};
                    SDL_RenderTexture(renderer, texture, nullptr, &frect);
                }
            }
        }
    }

    SDL_Color text_color = {255, 255, 255, 255};
    SDL_Surface* text_surface = TTF_RenderText_Solid_Wrapped(font_playinfo, ss.str().c_str(), 0, text_color, 0);
    SDL_Texture* text_texture = SDL_CreateTextureFromSurface(renderer, text_surface);
    SDL_FRect text_rect = {900, 220, (float)text_surface->w, (float)text_surface->h};
    SDL_RenderTexture(renderer, text_texture, nullptr, &text_rect);
    SDL_DestroySurface(text_surface);
    SDL_DestroyTexture(text_texture);

    for (auto& player : players) {
        JudgeInfo info = player.GetResult().GetLastJudgeInfo(LEFT);
        if (info.judge != JUDGE_YET) {
            ms_type elapsed = player.GetPlayMs() - info.ms;
            if (elapsed < 2000 && (info.judge == Judge::PGREAT || (elapsed / 10) % 3 == 1)) {

                SDL_Color text_color = color_judge_combo[info.judge];
                SDL_Surface* text_surface = TTF_RenderText_Solid(font_judge, (str_judge_combo[info.judge] + std::to_string(info.combo)).c_str(), 0, text_color);
                SDL_Texture* text_texture = SDL_CreateTextureFromSurface(renderer, text_surface);
                SDL_FRect text_rect = {judge_combo_x, judge_combo_y, (float)text_surface->w, (float)text_surface->h};
                SDL_RenderTexture(renderer, text_texture, nullptr, &text_rect);
                SDL_DestroySurface(text_surface);
                SDL_DestroyTexture(text_texture);
            }
        }
    }
}

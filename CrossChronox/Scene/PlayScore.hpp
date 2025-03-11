#pragma once

#include "pch.hpp"
#include "Scene.hpp"
#include "Score/Play/ScorePlayer.hpp"

class JudgeCombo {
    const ScorePlayer* player;
    sf::Text text;
    Side side = LEFT;

    ms_type last_ms = 0;
    size_t last_combo = 0;

  public:
    void Init(const ScorePlayer& player, Side side);
    const sf::Text* GetText();
};

class PlayScore : public Scene {
    std::vector<ScorePlayer> players;

    std::array<JudgeCombo, MAX_SIDE> judge_combos;

    sf::Texture background;
    sf::Sprite background_sprite;

    sf::Texture white_shape;
    sf::Sprite white_sprite;

    sf::Texture black_shape;
    sf::Sprite black_sprite;

    sf::Texture scr_shape;
    sf::Sprite scr_sprite;

    sf::Texture judgeline_shape;
    sf::Sprite judgeline_sprite;

    sf::Text text_fps;
    sf::Text text_info_play;

    sf::Clock clock_fps;
    int fps_count = 0;

    sf::Sprite* GetSpritePtr(lane_t lane) {
        switch (lane) {
        case 8:
            return &scr_sprite;
        case 1:
        case 3:
        case 5:
        case 7:
            return &white_sprite;
        case 2:
        case 4:
        case 6:
            return &black_sprite;
        default:
            return nullptr;
        }
    }

  public:
    void Init();
    void Deinit();
    Scene* Update();
    void Draw(sf::RenderTarget& render_target);
};

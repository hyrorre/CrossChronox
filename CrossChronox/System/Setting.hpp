#pragma once

#include "pch.hpp"

enum {
    WINDOWED,
    FULLSCREEN,
    BORDERLESS,
};

class Setting {
    int window_type = WINDOWED;
    bool save_resolution = true;
    int resolution_x = 1920;
    int resolution_y = 1080;
    bool save_window_size = true;
    int window_size_x = 1920;
    int window_size_y = 1080;
    bool save_window_pos = true;
    int window_pos_x = 400;
    int window_pos_y = 400;
    bool vsync = true;
    int max_fps = 0; // if max_fps is 0, fps is unlimited

    std::vector<std::string> song_paths;
    std::vector<std::string> table_urls;

  public:
    int GetWindowType() const {
        return window_type;
    }
    int GetResolutionX() const {
        return resolution_x;
    }
    int GetResolutionY() const {
        return resolution_y;
    }
    sf::Vector2i GetResolution() const {
        return sf::Vector2i(GetResolutionX(), GetResolutionY());
    }
    int GetWindowSizeX() const {
        return window_size_x;
    }
    int GetWindowSizeY() const {
        return window_size_y;
    }
    sf::Vector2i GetWindowSize() const {
        return sf::Vector2i(GetWindowSizeX(), GetWindowSizeY());
    }
    int GetWindowsPosX() const {
        return window_pos_x;
    }
    int GetWindowPosY() const {
        return window_pos_y;
    }
    sf::Vector2i GetWindowPos() const {
        return sf::Vector2i(GetWindowsPosX(), GetWindowPosY());
    }
    void SetWindowPos(const sf::Vector2i& value) {
        if (save_window_pos) {
            window_pos_x = value.x;
            window_pos_y = value.y;
        }
    }
    bool GetVsync() const {
        return vsync;
    }
    int GetMaxFps() const {
        return max_fps;
    }

    bool TryLoadFile(const std::string& filepath) {
        try {
            //auto str = serde::parse_file<serde::toml_v>(filepath);
            //*this = serde::deserialize<Setting>(str);
            return true;
        } catch (std::exception& e) {
            return false;
        }
    }

    void SaveFile(const std::string& filepath) {
        //std::ofstream ofs(filepath);
        //auto str = serde::serialize<serde::toml_v>(*this);
        //ofs << str;
    }

    //template <class Context>
    //constexpr static void serde(Context& context, Setting& value) {
    //    serde::serde_struct(context, value)
    //        .field(&Setting::window_type, "window_type")
    //        .field(&Setting::save_resolution, "save_resolution")
    //        .field(&Setting::resolution_x, "resolution_x")
    //        .field(&Setting::resolution_y, "resolution_y")
    //        .field(&Setting::save_window_size, "save_window_size")
    //        .field(&Setting::window_size_x, "window_size_x")
    //        .field(&Setting::window_size_y, "window_size_y")
    //        .field(&Setting::save_window_pos, "save_window_pos")
    //        .field(&Setting::window_pos_x, "window_pos_x")
    //        .field(&Setting::window_pos_y, "window_pos_y")
    //        .field(&Setting::vsync, "vsync")
    //        .field(&Setting::max_fps, "max_fps")
    //        .field(&Setting::song_paths, "song_paths")
    //        .field(&Setting::table_urls, "table_urls");
    //}
};

extern Setting setting;

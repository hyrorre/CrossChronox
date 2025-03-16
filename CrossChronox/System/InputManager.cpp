#include "InputManager.hpp"

namespace InputManager {

    using keyid_t = unsigned;

    const bool* sdl_keyboard_states = nullptr;

    bool IsKeyPressed(keyid_t keyid) {
        return sdl_keyboard_states[keyid];
        // if (keyid < 1000) { // keyboard
        //     return sf::Keyboard::isKeyPressed(static_cast<sf::Keyboard::Key>(keyid));
        // } else { // joystick
        //     unsigned joyid = (keyid / 1000) - 1;
        //     if (int axis = (keyid / 100) % 10) {
        //         --axis;
        //         auto pos = sf::Joystick::getAxisPosition(joyid, static_cast<sf::Joystick::Axis>(axis));
        //         if (keyid % 10)
        //             return (80 < pos);
        //         else
        //             return (pos < -80);
        //     } else
        //         return sf::Joystick::isButtonPressed(joyid, keyid % 100);
        // }
        // return false;
    };

    using keymap_t = std::unordered_map<std::string, std::vector<keyid_t>>;

    using keyfuncmap_t = std::unordered_map<std::string, std::vector<std::string>>;

    std::unordered_map<keyid_t, KeyState> key_states;

    struct Mode {
        keymap_t keymap;
        keyfuncmap_t keyfuncmap;
    };

    using modemap_t = std::unordered_map<std::string, Mode>;
    modemap_t modemap;

    Mode* now_mode = nullptr;

    void LoadConfig(const std::string& config_path) {
        auto root = toml::parse(config_path).as_table();
        for (auto& mode_pair : root) {
            auto& mode = modemap[mode_pair.first];
            auto& mode_data = mode_pair.second.as_table();
            auto keymap = mode_data.at("Key").as_table();
            for (auto& key_pair : keymap) {
                auto& keyids = mode.keymap[key_pair.first];
                for (auto keyid : key_pair.second.as_array()) {
                    keyid_t tmp = static_cast<keyid_t>(keyid.as_integer());
                    keyids.emplace_back(tmp);
                    key_states[tmp];
                }
            }
            auto& keyfuncmap = mode_data.at("KeyFunc").as_table();
            for (auto& func_pair : keyfuncmap) {
                auto& keynames = mode.keyfuncmap[func_pair.first];
                for (auto& keyname : func_pair.second.as_array()) {
                    keynames.emplace_back(keyname.as_string());
                }
            }
        }
    }

    void SetMode(const std::string& mode) {
        try {
            now_mode = &modemap.at(mode);
        } catch (std::out_of_range& e) {
            throw std::runtime_error(std::string("The mode named " + mode + " was not found or configured.\n") + e.what());
        }
    }

    void Update() {
        sdl_keyboard_states = SDL_GetKeyboardState(nullptr);
        for (auto& key_state : key_states) {
            const keyid_t& keyid = key_state.first;
            auto& state = key_state.second;

            if (0 < state.now) { // last frame, key was pressed
                if (IsKeyPressed(keyid))
                    ++state.now;
                else {
                    state.now = -1;
                    // state.last_switched_ms = now_ms;
                }
            } else { // last frame, key was released
                if (IsKeyPressed(keyid)) {
                    state.now = 1;
                    // state.last_switched_ms = now_ms;
                } else
                    --state.now;
            }
            if (state.now == 2 || state.now == -2) {
                state.last_switched_ms = lastframe_ms;
            }
        }
    }

    KeyState GetKeyState(const std::string& key_str) {
        assert(now_mode);
        KeyState ret;
        for (auto keyid : now_mode->keymap[key_str]) {
            auto& state = key_states[keyid];
            // ret.now = std::max(ret.now, state.now);
            // ret.last = std::max(ret.last, state.last);
            if (ret.now < state.now) {
                ret = state;
            }
        }
        return ret;
    }

    KeyState GetKeyFuncState(const std::string& key_func_str) {
        assert(now_mode);
        KeyState ret;
        for (auto& key_str : now_mode->keyfuncmap[key_func_str]) {
            KeyState state = GetKeyState(key_str);
            // ret.now = std::max(ret.now, state.now);
            // ret.last = std::max(ret.last, state.last);
            if (ret.now < state.now) {
                ret = state;
            }
        }
        return ret;
    }
} // namespace InputManager

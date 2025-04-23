#pragma once

// C std
#include <cmath>
#include <cwchar>

// C++ std
#include <algorithm>
#include <array>
#include <bitset>
#include <chrono>
#include <codecvt>
#include <exception>
#include <filesystem>
#include <forward_list>
#include <fstream>
#include <locale>
#include <memory>
#include <numeric>
#include <queue>
#include <random>
#include <stack>
#include <stdexcept>
#include <system_error>
#include <type_traits>
#include <typeinfo>
#include <unordered_map>
#include <utility>
#include <vector>

// boost
#include <boost/algorithm/string.hpp>

// Crypto++ (cryptopp)
#define CRYPTOPP_ENABLE_NAMESPACE_WEAK 1 // to use md5
#include <cryptopp/hex.h>
#include <cryptopp/md5.h>

// libiconv (iconv)
#include <iconv.h>

// SDL
#include <SDL3/SDL.h>
#include <SDL3_image/SDL_image.h>
#include <SDL3_ttf/SDL_ttf.h>

// SFML
#include <SFML/Audio.hpp>

// toml11
#include <toml.hpp>

// Windows
#if defined(_WIN64) || defined(_WIN32) // if Windows
#define WIN32_LEAN_AND_MEAN
#define NOMINMAX
#include <windows.h>
#endif

// using (namespace)
namespace fs = std::filesystem;

// global funcs
template <class T>
struct ptr_less {
    bool operator()(const std::unique_ptr<T>& a, const std::unique_ptr<T>& b) const {
        return *a < *b;
    }
    bool operator()(const T* a, const T* b) const {
        return *a < *b;
    }
};

// global variables
extern std::mt19937 mt_rand;

// macros
#define _CRT_SECURE_NO_WARNINGS

// pragma
#pragma execution_character_set("utf-8")

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

// SFML
#include <SFML/Audio.hpp>
#include <SFML/Config.hpp>
#include <SFML/Graphics.hpp>
#include <SFML/Main.hpp>
#include <SFML/Network.hpp>
#include <SFML/OpenGL.hpp>
#include <SFML/System.hpp>
#include <SFML/Window.hpp>

// toml11
#include <toml.hpp>

// picojson
#define PICOJSON_USE_INT64
#include <picojson/picojson.h>

// Crypto++ (cryptopp)
#define CRYPTOPP_ENABLE_NAMESPACE_WEAK 1 // to use md5
#include <cryptopp/hex.h>
#include <cryptopp/md5.h>

// boost
#include <boost/algorithm/string.hpp>

// iconv
#include <iconv.h>

// Windows
#if defined(_WIN64) || defined(_WIN32) // if Windows
#define WIN32_LEAN_AND_MEAN
#define NOGDICAPMASKS
#define NOVIRTUALKEYCODES
#define NOWINMESSAGES
#define NOWINSTYLES
#define NOSYSMETRICS
#define NOMENUS
#define NOICONS
#define NOKEYSTATES
#define NOSYSCOMMANDS
#define NORASTEROPS
#define NOSHOWWINDOW
#define OEMRESOURCE
#define NOATOM
#define NOCLIPBOARD
#define NOCOLOR
#define NOCTLMGR
#define NODRAWTEXT
#define NOGDI
#define NOKERNEL
#define NOUSER
#define NONLS
#define NOMB
#define NOMEMMGR
#define NOMETAFILE
#define NOMINMAX
#define NOMSG
#define NOOPENFILE
#define NOSCROLL
#define NOSERVICE
#define NOSOUND
#define NOTEXTMETRIC
#define NOWH
#define NOWINOFFSETS
#define NOCOMM
#define NOKANJI
#define NOHELP
#define NOPROFILER
#define NODEFERWINDOWPOS
#define NOMCX
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

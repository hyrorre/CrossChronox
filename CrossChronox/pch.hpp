﻿#pragma once

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
#include <boost/algorithm/algorithm.hpp>
#include <boost/algorithm/clamp.hpp>
#include <boost/algorithm/string.hpp>
#include <boost/assign.hpp>
#include <boost/bind.hpp>
#include <boost/function.hpp>
#include <boost/functional.hpp>
#include <boost/integer_traits.hpp>
#include <boost/optional.hpp>
#include <boost/range.hpp>
#include <boost/range/algorithm/binary_search.hpp>
#include <boost/range/algorithm/lower_bound.hpp>
#include <boost/range/algorithm/sort.hpp>
#include <boost/range/algorithm/upper_bound.hpp>
#include <boost/range/algorithm_ext.hpp>
#include <boost/rational.hpp>
#include <boost/swap.hpp>
#include <boost/utility.hpp>
#include <boost/utility/string_ref.hpp>
#include <boost/variant.hpp>

// iconv
#include <iconv.h>

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

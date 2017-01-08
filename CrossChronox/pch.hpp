//
//  pch.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 10/7/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#ifndef pch_hpp
#define pch_hpp

//SFML
#include <SFML/Audio.hpp>
#include <SFML/Config.hpp>
#include <SFML/Graphics.hpp>
#include <SFML/Main.hpp>
#include <SFML/Network.hpp>
#include <SFML/OpenGL.hpp>
#include <SFML/System.hpp>
#include <SFML/Window.hpp>

//sfeMovie
#include <sfeMovie/Movie.hpp>

//Qt
//#include <QApplication>
//#include <QMessageBox>
//#include <QTextCodec>

//jsoncpp
//#include <json/json.h>

//Crypto++ (cryptopp)
#define CRYPTOPP_ENABLE_NAMESPACE_WEAK 1 //to use md5
#include <cryptopp/hex.h>
#include <cryptopp/md5.h>

//boost
#include <boost/filesystem.hpp>
#include <boost/utility.hpp>
#include <boost/utility/string_ref.hpp>
#include <boost/variant.hpp>
#include <boost/function.hpp>
#include <boost/algorithm/string.hpp>
#include <boost/bind.hpp>
#include <boost/rational.hpp>
#include <boost/integer_traits.hpp>
#include <boost/functional.hpp>
#include <boost/assign.hpp>
#include <boost/swap.hpp>
#include <boost/optional.hpp>
#include <boost/algorithm/algorithm.hpp>
#include <boost/algorithm/clamp.hpp>
#include <boost/range/algorithm_ext.hpp>
#include <boost/range/algorithm/binary_search.hpp>
#include <boost/range/algorithm/lower_bound.hpp>
#include <boost/range/algorithm/upper_bound.hpp>
#include <boost/range/algorithm/sort.hpp>
#include <boost/range.hpp>


//C std
#include <cmath>

//C++ std
#include <forward_list>
#include <vector>
#include <unordered_map>
#include <array>
#include <memory>
#include <exception>
#include <stdexcept>
#include <system_error>
#include <algorithm>
#include <stack>
#include <queue>
#include <locale>
#include <chrono>
#include <random>
#include <type_traits>
#include <typeinfo>
#include <fstream>
#include <utility>
#include <numeric>

//using (namespace)
namespace fs = boost::filesystem;

//global funcs
template<class T>
struct ptr_less{
	bool operator()(const std::unique_ptr<T>& a, const std::unique_ptr<T>& b) const{
        return *a < *b;
    }
};

//global variables
extern std::mt19937 mt_rand;

#endif /* pch_hpp */

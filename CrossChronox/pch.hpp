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
#include <QtWidgets/QApplication>
#include <QtWidgets/QMessageBox>
#include <QtCore/QTextCodec>

//jsoncpp
#include <json/json.h>

//Crypto++ (cryptopp)
#define CRYPTOPP_ENABLE_NAMESPACE_WEAK 1 //to use md5
#include <cryptopp/hex.h>
#include <cryptopp/md5.h>

//boost
#include <boost/any.hpp>
#include <boost/filesystem.hpp>
#include <boost/utility.hpp>
#include <boost/utility/string_ref.hpp>
#include <boost/ptr_container/ptr_list.hpp>
#include <boost/ptr_container/ptr_vector.hpp>
#include <boost/ptr_container/ptr_unordered_map.hpp>
#include <boost/variant.hpp>
#include <boost/function.hpp>
#include <boost/xpressive/xpressive.hpp>
#include <boost/algorithm/string.hpp>
#include <boost/smart_ptr.hpp>
#include <boost/bind.hpp>
#include <boost/rational.hpp>
#include <boost/integer_traits.hpp>
#include <boost/functional.hpp>
#include <boost/assign.hpp>
#include <boost/swap.hpp>
#include <boost/optional.hpp>
#include <boost/algorithm/algorithm.hpp>
#include <boost/algorithm/clamp.hpp>


//C std
#include <cmath>

//C++ std
#include <list>
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

//using
namespace fs = boost::filesystem;


#endif /* pch_hpp */

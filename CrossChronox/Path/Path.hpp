////////////////////////////////////////////////////////////
//
// SFML - Simple and Fast Multimedia Library
// Copyright (C) 2007-2016 Marco Antognini (antognini.marco@gmail.com),
//                         Laurent Gomila (laurent@sfml-dev.org)
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it freely,
// subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented;
//    you must not claim that you wrote the original software.
//    If you use this software in a product, an acknowledgment
//    in the product documentation would be appreciated but is not required.
//
// 2. Altered source versions must be plainly marked as such,
//    and must not be misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//
////////////////////////////////////////////////////////////

#ifndef PATH_HPP
#define PATH_HPP

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include "pch.hpp"

////////////////////////////////////////////////////////////
///
/// The path to the resource or document (etc.) folder associate
/// with the main bundle or an empty string is there is no bundle.
///
////////////////////////////////////////////////////////////

namespace Path{
	extern const fs::path& resource;
	extern const fs::path& font;
	extern const fs::path& appdata;
	extern const fs::path& cache;
	
	void Init(void);
	
	class InitError : public std::runtime_error{
	public:
		InitError(const std::string& msg): std::runtime_error(msg){}
		virtual ~InitError(){}
	};
}

#endif

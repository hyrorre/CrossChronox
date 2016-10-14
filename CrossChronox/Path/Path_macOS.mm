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

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include "Path.hpp"
#import <Foundation/Foundation.h>

////////////////////////////////////////////////////////////
fs::path resourcePath(void){
	NSAutoreleasePool* pool = [[NSAutoreleasePool alloc] init];

	std::string rpath;
	NSBundle* bundle = [NSBundle mainBundle];

	if(bundle == nil){
#ifdef DEBUG
		NSLog(@"bundle is nil... thus no resources path can be found.");
#endif
		throw Path::InitError("Bundle is nil. Please run \"CrossChronox\"");
	}
	else{
		NSString* path = [bundle resourcePath];
		rpath = [path UTF8String];
	}

	[pool drain];

	return rpath;
}

//std::string fontPath(void)
//{
//	return "/Library/Fonts/";
//}

fs::path appdataPath(void){
	fs::path apath;
	NSBundle* bundle = [NSBundle mainBundle];
	
	if(bundle == nil){
#ifdef DEBUG
		NSLog(@"bundle is nil... thus no resources path can be found.");
#endif
	}
	else{
		NSString* path = [bundle bundlePath];
		apath = [path UTF8String];
		
		while(apath.has_parent_path()){
			apath = apath.parent_path();
			fs::path tmp_path = apath / "CrossChronoxData";
			if(fs::exists(tmp_path)) return tmp_path;
		}
	}
	throw Path::InitError("\"CrossChronoxData\" folder was not found.");
}


fs::path cachePath(void){
	NSAutoreleasePool* pool = [[NSAutoreleasePool alloc] init];
	NSArray *paths = NSSearchPathForDirectoriesInDomains(NSCachesDirectory, NSUserDomainMask, YES);
	NSString *cachesDirPath = [paths objectAtIndex:0];
	fs::path cpath = [cachesDirPath UTF8String] + std::string("/CrossChronox");
	if(!fs::exists(cpath)){
		fs::create_directories(cpath);
	}
	[pool drain];
	return cpath;
}


namespace Path{
	fs::path resource_;
	fs::path font_;
	fs::path appdata_;
	fs::path cache_;
	
	const fs::path& resource = resource_;
	const fs::path& font = font_;
	const fs::path& appdata = appdata_;
	const fs::path& cache = cache_;
	
	
	void Init(void){
		resource_ = resourcePath();
		font_ = "/Library/Fonts/";
		appdata_ = appdataPath();
		cache_ = cachePath();
	}
}



#include "MessageBox.hpp"
#import <AppKit/NSAlert.h>

int MessageBoxA(sf::WindowHandle handle, const char* text, const char* title, unsigned flag){
	NSString *nstitle = [ [ NSString alloc ] initWithUTF8String:title ];
	NSString *nstext = [ [ NSString alloc ] initWithUTF8String:text ];
	NSAlert* alert = [[NSAlert alloc] init];
	[ alert setMessageText:nstitle ];
	[ alert setInformativeText:nstext];
	
	if(flag & MB_YESNO) [alert addButtonWithTitle:@"はい"];
	
	if(flag & MB_ICONERROR) [ alert setAlertStyle:NSAlertStyleCritical ];
	else if(flag & MB_ICONWARNING) [ alert setAlertStyle:NSAlertStyleWarning ];
	else if(flag & MB_ICONINFORMATION) [ alert setAlertStyle:NSAlertStyleInformational ];
	
	return static_cast<int>([alert runModal]);
}

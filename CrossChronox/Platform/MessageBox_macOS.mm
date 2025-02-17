#include "MessageBox.hpp"
#import <AppKit/NSAlert.h>

int MBox(sf::WindowHandle handle, const char* title, const char* text, unsigned flag){
	NSString *nstitle = [ [ NSString alloc ] initWithUTF8String:title ];
	NSString *nstext = [ [ NSString alloc ] initWithUTF8String:text ];
	NSAlert* alert = [[NSAlert alloc] init];
	[ alert setMessageText:nstitle ];
	[ alert setInformativeText:nstext];
	
	if(flag & MBOX_YESNO) [alert addButtonWithTitle:@"はい"];
	
	if(flag & MBOX_ICONERROR) [ alert setAlertStyle:NSCriticalAlertStyle ];
	else if(flag & MBOX_ICONWARNING) [ alert setAlertStyle:NSWarningAlertStyle ];
	else if(flag & MBOX_ICONINFORMATION) [ alert setAlertStyle:NSInformationalAlertStyle ];
	
	return static_cast<int>([alert runModal]);
}

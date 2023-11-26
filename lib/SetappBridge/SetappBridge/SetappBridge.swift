import Setapp

@_cdecl("startPlugin")
public func startPlugin() -> Void {
    SetappManager.logLevel = .info
}

@_cdecl("showReleaseNotesWindowIfNeeded")
public func showReleaseNotesWindowIfNeeded() -> Void {
    SetappManager.shared.showReleaseNotesWindowIfNeeded()
}

@_cdecl("showReleaseNotesWindow")
public func showReleaseNotesWindow() -> Void {
    SetappManager.shared.showReleaseNotesWindow()
}

@_cdecl("askUserToShareEmail")
public func askUserToShareEmail() -> Void {
    SetappManager.shared.askUserToShareEmail()
}

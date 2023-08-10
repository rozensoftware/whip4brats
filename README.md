# Whip for Brats

v 0.1.0

This is a mini system for monitoring the time children spend in front of a computer screen during the day (and night). Unfortunately, children and early adolescents have no restraint in sitting in front of the computer and playing computer games. This is another remedy for parents, who can indicate how long the computer will function for a child.
After a certain amount of time, there is a screen lock, which prevents further play (or makes it very difficult).

## Screenshots

![Whip for Brats GUI App](https://github.com/rozensoftware/whip4brats/blob/master/whip4bratsgui.png)

## Features

- A weekly calendar indicating the range of available time: from an hour with minutes, to an hour with minutes
- Disabling/Enabling surveillance
- Three-level system with supervisor service, command execution application and GUI app for setting time and other parameters
- Temporary removal of the lock from 5 minutes to 1 hour
- Multilingual (currently English and Polish - mostly translated)

## Building

Build everything. Publish Whip4BratsGUI (.NET) app.

Copy all files listed below to your folder:

- brat-server.exe
- executor.exe
- BratLocker.exe
- 'publish' folder of exported .NET files (all .NET 7 framework and app files)

## Installation

Run the Whip4BratsGUI module and configure the parental settings, i.e.: the parent account password, child account name and password.
(The application password is the password of the parent's account in the system or '1234' at the first start).
A temporary password of '1234' will be created, which will be replaced with the password of the parent account after it is configured on the Password Setup Screen.

Important! The child's account should not be in the Administrator group of the computer, or the supervision will be possible to remove by the child. On the other hand, the parent account must belong to the computer Administrator group.
Next, define the time when the child can play on the computer. There is an option which can deactivate time limits.

Once you have made all the settings in the Whip4BratsGUI, you can start the service which can be done in Whip4BratsGUI app.

To install (register) service manually run the below command with elevated privileges:

```powershell
./brat-server --register
```

Unregister:

```powershell
./brat-server --unregister
```

For help type:

```powershell
./brat-server --help
```

Start service from The Service Management.

## Locked Screen

When the computer screen has been locked, only the parent can unlock it by entering the password on the lock screen and selecting additional time.
The screen is not locked completely, but the locking is so troublesome that further work is pointless.

## The Project

The system consists of several components:

- Windows service that monitors user time (brat-server) - written in Rust
- TCP Command Server (Executor) - written in Rust
- Screen Locker - written in C++20
- Whip4BratsGUI (Application for setting time and other parameters) - written in C# (.NET 7)

Visual Studio 2022 and VS Code have been used to write this software. Probably VS Code could be used alone but I haven't had a chance to check it.
There is no publish script to make an installer so far. All exe files must be in one folder to work correctly.

Bart-server logs might be seen in Windows Event Viewer (Application node).

## TODO

These are some ideas to implement in future:

- Write applications to remotely change the available time and monitor computer usage from the web browser.
- Block the launch of selected applications.
- Reporting of computer usage in given time intervals.
- Installer for easy software setup.

## License

This project is licensed under either of

Apache License, Version 2.0, (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>) MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>) at your option.

## Contributing / Feedback

This project is not finished and is waiting to be developed further.
Several technologies were used to write it:

- Rust (Windows Service, TCP Server)
- C/C++ (Windows API)
- C# (WinUI)

You are sure to find something in it for you.
If you want to contribute, you are more than welcome to be a part of the project! Try to share your thoughts first! Feel free to open a new issue if you want to discuss new ideas.

Any kind of feedback is welcome!

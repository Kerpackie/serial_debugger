=========================
Serial Debugger v0.2.0
=========================

Thank you for downloading Serial Debugger!

-------------------------
TO INSTALL
-------------------------

1.  Unzip this package to a folder on your computer.

2.  Run the installer for your operating system:
    - WINDOWS: Double-click the `install.bat` file.
      (You may need to right-click and "Run as administrator").

    - MACOS / LINUX: Open a terminal, navigate into this folder, and run:
      ./install.sh

3.  Restart your terminal window. The installation is complete.

-------------------------
HOW TO USE
-------------------------

Open a new terminal and use the `serial_debugger` command.

# Basic Usage (print to console)
serial_debugger <PORT> <BaudRate>
Example: serial_debugger COM3 9600

# Log to a File (print to console AND save to file)
serial_debugger <PORT> <BaudRate> -o <FileName>
Example: serial_debugger /dev/ttyUSB0 9600 -o log.txt

-------------------------
TO UNINSTALL
-------------------------

1.  Navigate back to this folder.

2.  Run the uninstaller for your operating system:
    - WINDOWS: Double-click the `uninstall.bat` file.
      (You may need to right-click and "Run as administrator").

    - MACOS / LINUX: Open a terminal and run:
      ./uninstall.sh

3.  Restart your terminal window to finalize the uninstallation.
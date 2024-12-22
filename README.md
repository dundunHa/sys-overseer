# sys-overseer
 **sys-overseer is a system status monitoring tool that allows you to observe CPU, memory, and network usage**.

# Feature
 - watch CPU usage
 - watch Memory usage
 - watch Network usage

 # Other
 
 If you see a security warning, there are two ways to resolve it:
 
Through System Preferences:

Open "System Preferences" > "Security & Privacy" > "General".
Click "Open Anyway" to allow the app to run.
Through Terminal:

Open Terminal.
Run the command: xattr -d com.apple.quarantine /path/to/your/app.
Replace /path/to/your/app with the actual path to the application.
Why does this warning appear?
This is part of macOS's security mechanism designed to protect users from potential malware. The warning appears because our app has not yet been certified by Apple, but this does not affect its functionality.

You may also encounter this issue when running on Windows. Please allow the program to run.

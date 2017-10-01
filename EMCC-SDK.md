# Installing Emscripten SDK

- Need brew for MacOS

## Install cmake

Mac
```
brew install cmake
```

Linux
```
sudo apt-get install cmake          # Debian Linux
```

Windows

- to be updated

## Install SDK

### Linux/Mac

```
 cd ~
 wget https://s3.amazonaws.com/mozilla-games/emscripten/releases/emsdk-portable.tar.gz
 tar -xvf emsdk-portable.tar.gz
 cd emsdk-portable
 ./emsdk update
 ./emsdk install sdk-incoming-64bit
```

### Windows

use instruction:
https://kripken.github.io/emscripten-site/docs/getting_started/downloads.html#platform-notes-installation-instructions-sdk

Download, unpack to c:\emcc and hose command should be enough:

```
# Fetch the latest registry of available tools.
./emsdk update

# Download and install the latest SDK tools.
./emsdk install latest

# Make the "latest" SDK "active" for the current user. (writes ~/.emscripten file)
./emsdk activate latest
```

`emcc --version` should show:

```
C:\>emcc --version
emcc (Emscripten gcc/clang-like replacement) 1.37.21 ()
Copyright (C) 2014 the Emscripten authors (see AUTHORS.txt)
This is free and open source software under the MIT license.
There is NO warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
```

Also install latest node-js from https://nodejs.org/en/ using the installer

better to use node-js console after that

then manually the following into the path:

c:\emcc\clang\e1.37.21_64bit;c:\emcc;c:\emcc\python\2.7.5.3_64bit;c:\emcc\java\7.45_64bit\bin;c:\emcc\emscripten\1.37.21

also edit c:\emcc\emscripten\1.37.21\emcc.bat so it contains only:

```
@echo off 
python "c:\emcc\emscripten\1.37.21\emcc.py" %*
```

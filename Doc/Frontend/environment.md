# Development Environment Setup

## Windows Subsystem for Linux - Ubuntu

The frontend development environment utilizes Ubuntu running on Windows Subsystem for Linux (WSL).  
The following instructions are for setting up the development environment on Windows 10.

## Install Guide:

### Install Ubuntu:

Install Ubuntu from the Microsoft Store. Recommended version: Ubuntu 22.04.2 LTS  
DO NOT use the root user for development. There are known issues with rust and cargo when using the root user.

### Install Rust & Cargo:

`curl https://sh.rustup.rs -sSf | sh`

### Reload .basrc:

`source ~/.bashrc`

### Install WebAssembly Target:

`rustup target add wasm32-unknown-unknown`

### Install Trunk:

`cargo install --locked trunk`

### Install libmysqlclient-dev (for linux & WSL):

`sudo apt update`

`sudo apt install libmysqlclient-dev`

### Install diesel_cli (optional for frontend work, but recommended - for database migrations):

`cargo install diesel_cli --no-default-features --features mysql`

### Install opensll:

`sudo apt install openssl`

`sudo apt install libssl-dev`

### Install pkg-config:

`sudo apt install pkg-config`

## WSL & VSCode Setup Guide:

### Install Vscode on Windows (in a elevated windows shell):

`winget source update`

`winget install Microsoft.VisualStudioCode`

### Install VSCode Server on WSL (in a Ubuntu-WSL shell):

`code`

### Clone The Repository:

`git clone <url> <directory_name>`

### Open VSCode From WSL:

For the best intellisense experience, open VSCode from the project directory you wish to work on.  
For the frontend, navigate into either the studentpage or adminpage directory.  
Then run the following command:

`code .` click yes to install WSL extension(s)

### Install The Following Extensions:

`Rust Extension Pack`

`Rust Syntax`

`Rust Doc Viewer`

`Cargo`

`Better TOML`

`Auto Close Tag`

`Auto Rename Tag`

`ayu` a theme that works well with rust, ayu mirage

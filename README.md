# Password-generator

## 1. Description

A CLI password generator

## 2. Features
    password-generator 0.1.0
    Lorenzo Costa <http://www.costa86.tech>
    A CLI password generator 🔐

    USAGE:
        password-generator [OPTIONS]

    OPTIONS:
        -q, --quantity <Number>             Passwords to generate [default: 5]
        -s, --size <Number>                 Size/Length of password [default: 8]
        -n, --numbers <Yes/No>              Include numbers [default: y] [possible values: y, n]
        -l, --lowercase-letters <Yes/No>    Include lowercase letters [default: y] [possible values: y,
                                            n]
        -u, --uppercase-letters <Yes/No>    Include uppercase letters [default: y] [possible values: y,
                                            n]
        -y, --symbols <Yes/No>              Include symbols [default: y] [possible values: y, n]
        -h, --help                          Print help information
        -V, --version                       Print version information

## 3. Instalation
### 3.1 Cargo

    cargo install password-generator

### 3.2 Ready-to-use executable

|OS|Architecture| File*|
|--|--|--|
|Linux|x86_64|[password-generator](https://github.com/costa86/safepass/blob/master/password-generator)|

*Make sure you've granted executable permissions to it

    ./password-generator


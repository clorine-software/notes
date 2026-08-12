<h1 align="center">Clorine Notes</h1>

## About

- notes - простая CLI программа для ведения записок и задач, написанная на rust

## Features && Info

- Создание заметок с содержимым, закрепление, вывод списка заметок и их контента в консоль

- Заметки хранятся в ron файле, конфиг (стили) в toml

## Usage

```sh
notes --help # Вывод всех комманд

notes list # Вывод списка заметок без декораций
notes display # Вывод списка заметок с декорациями, описанными в ~/.config/notes/config.toml

notes new <name> [CONTENT] [OPTIONS] # Создание заметки по имени и содержимому. Флаг --special - заметка будет создана в группе special
notes cat <index> [OPTIONS] # Вывод названия, содержимого заметки по индексу. Флаг --special - указание группы special
notes delete <index> [OPTIONS] # Удаление заметки по индексу. Флаг --special - будет удалена заметка из группы special. Флаг --force - программа не запросит подтверждение

notes print-json # Вывод БД в JSON, с помощью этой команды вы сможете сделать свой интерфейс

```

## Installation

### Linux

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/clorine-software/notes/releases/download/v1.2.4/notes-installer.sh | sh
```

Или скачайте версию из релизов

### Windows

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/clorine-software/notes/releases/download/v1.2.4/notes-installer.ps1 | iex"
```

Или скачайте версию из релизов

### MacOS

Скачайте версию из релизов



# WorkflowEngine

Визуальный редактор игровой логики на основе узлов. Соединяйте блоки вместо написания кода, расширяйте с помощью Lua-скриптов.

## Что это

Инструмент для построения игровой логики путём перетаскивания узлов и соединения портов.

Основная идея: разбить игровую логику на блоки (узлы), каждый блок — это Lua-скрипт, блоки передают данные через соединения. Используйте для:

- Пошаговых боевых систем
- Расчёта навыков/баффов
- Конечных автоматов
- Любой логики, представимой как поток данных
<img width="1403" height="863" alt="image" src="https://github.com/user-attachments/assets/7201603f-72a7-4035-b66b-c1bc7106df32" />

https://github.com/user-attachments/assets/08793b5b-d584-44a1-b641-9e8912ce3061

## Быстрый старт

```bash
# Клонировать
git clone https://github.com/LegnaOS/workflow-game.git
cd workflow-game

# Сборка и запуск
cargo run --release

# Или скачайте из Releases
```

После запуска:
1. Левая панель — список блоков, двойной клик для добавления
2. Перетаскивайте порты для создания соединений
3. Правая панель — редактирование свойств блока
4. Ctrl+S — сохранить, Ctrl+O — открыть

## Форматы файлов

| Расширение | Описание |
|------------|----------|
| `.L` | Открытый JSON, редактируемый |
| `.LZ` | Зашифрованный, требует пароль |
| `.dist.L` | Дистрибутив, только чтение |
| `.dist.LZ` | Зашифрованный дистрибутив |

## Создание блоков

Блоки — это Lua-скрипты. Поместите их в папку `scripts/`, автозагрузка с горячей перезагрузкой.

Минимальный пример:

```lua
return {
    meta = {
        id = "my.double",
        name = "Удвоение",
        category = "Математика",
        color = "#FF5722"
    },
    inputs = {
        { id = "value", name = "Вход", type = "number", default = 0 }
    },
    outputs = {
        { id = "result", name = "Результат", type = "number", default = 0 }
    },
    execute = function(self, inputs)
        return { result = inputs.value * 2 }
    end
}
```

Подробнее: [docs/BLOCK_DEVELOPMENT_RU.md](docs/BLOCK_DEVELOPMENT_RU.md)

## Встроенные блоки

```
scripts/
├── game/          # Игра
│   ├── character  # Персонаж (характеристики)
│   ├── monster    # Монстр
│   ├── attack     # Расчёт атаки
│   └── fireball   # Огненный шар
├── logic/         # Логика
│   ├── branch     # Условное ветвление
│   ├── compare    # Сравнение
│   └── selector   # Селектор
├── math/          # Математика
│   ├── add        # Сложение
│   ├── multiply   # Умножение
│   └── calc       # Выражение
└── util/          # Утилиты
    ├── splitter   # Разделитель
    ├── merger     # Объединитель
    └── switch     # Переключатель
```

## Сборка

Требуется Rust 1.70+

```bash
# Разработка
cargo run

# Релиз
./build.sh all

# Одна платформа
./build.sh mac
./build.sh mac-intel
./build.sh windows
```

Результат в папке `dist/`.

## Структура проекта

```
src/
├── main.rs           # Точка входа, шрифты
├── app.rs            # Основная логика
├── script/           # Lua-движок
│   ├── loader.rs     # Кодировка (UTF-8/GBK)
│   ├── registry.rs   # Реестр блоков
│   └── executor.rs   # Исполнитель
├── workflow/         # Ядро workflow
│   ├── graph.rs      # Структура графа
│   ├── block.rs      # Определение блока
│   ├── connection.rs # Соединения
│   └── storage.rs    # Хранилище
└── ui/               # Компоненты UI
    ├── canvas.rs     # Холст
    └── block_widget.rs
```

## Технологии

- **Rust** — ядро
- **egui/eframe** — GUI
- **mlua** — Lua 5.4
- **serde** — сериализация

## Лицензия

MIT


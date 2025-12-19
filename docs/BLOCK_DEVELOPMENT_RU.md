# Руководство по разработке блоков

> Блок — это базовая единица выполнения движка workflow. Каждый блок — это Lua-скрипт.
> Поместите в папку `scripts/`, поддерживается горячая перезагрузка.

---

## Содержание

- [Быстрый старт](#быстрый-старт)
- [Структура скрипта](#структура-скрипта)
- [Типы данных](#типы-данных)
- [Основные концепции](#основные-концепции)
- [Интерактивные виджеты](#интерактивные-виджеты)
- [Система анимации](#система-анимации)
- [Разработка USB](#разработка-usb)
- [Лучшие практики](#лучшие-практики)

---

## Быстрый старт

**Минимальный пример** — Создайте `scripts/my/double.lua`:

```lua
return {
    meta = {
        id = "my.double",
        name = "Удвоение",
        category = "Мои",
        color = "#FF5722"
    },
    inputs = {
        { id = "value", name = "Вход", type = "number", default = 0 }
    },
    outputs = {
        { id = "result", name = "Результат", type = "number" }
    },
    execute = function(self, inputs)
        return { result = inputs.value * 2 }
    end
}
```

Сразу появится в левой панели IDE в категории "Мои".

---

## Структура скрипта

```lua
return {
    -- ═══════════════════════════════════════════════════════════
    -- Метаданные (обязательно)
    -- ═══════════════════════════════════════════════════════════
    meta = {
        id = "category.name",       -- Уникальный идентификатор (обязательно)
        name = "Отображаемое имя",  -- Заголовок блока
        category = "Категория",     -- Категория в левой панели
        description = "Подсказка",  -- Описание при наведении
        color = "#4CAF50",          -- Цвет заголовка
        hideable = false,           -- Скрывать в режиме предпросмотра (опционально)
        widget = nil                -- Тип интерактивного виджета (опционально)
    },

    -- ═══════════════════════════════════════════════════════════
    -- Свойства (опционально) — Редактируются в правой панели
    -- ═══════════════════════════════════════════════════════════
    properties = {
        { id = "damage", name = "Урон", type = "number", default = 10, min = 0, max = 999 },
        { id = "name", name = "Имя", type = "string", default = "Герой" },
        { id = "active", name = "Активен", type = "boolean", default = true }
    },

    -- ═══════════════════════════════════════════════════════════
    -- Входные порты (опционально) — Жёлтые точки слева
    -- ═══════════════════════════════════════════════════════════
    inputs = {
        { id = "trigger", name = "Триггер", type = "event" },
        { id = "value", name = "Значение", type = "number", default = 0 }
    },

    -- ═══════════════════════════════════════════════════════════
    -- Выходные порты (опционально) — Синие точки справа
    -- ═══════════════════════════════════════════════════════════
    outputs = {
        { id = "result", name = "Результат", type = "number" },
        { id = "done", name = "Готово", type = "event" }
    },

    -- ═══════════════════════════════════════════════════════════
    -- Функция выполнения (обязательно) — Основная логика
    -- ═══════════════════════════════════════════════════════════
    execute = function(self, inputs)
        -- self.properties  → Значения свойств
        -- self.state       → Постоянное состояние (между выполнениями)
        -- inputs           → Значения входных портов

        return {
            result = inputs.value * 2,
            done = true  -- тип event: не-nil = сработало
        }
    end
}
```

---

## Типы данных

| Тип | Lua-тип | Цвет порта | Описание |
|-----|---------|------------|----------|
| `number` | number | Синий | Числовое значение |
| `string` | string | Зелёный | Текстовая строка |
| `boolean` | boolean | Оранжевый | Истина/ложь |
| `event` | any/nil | Жёлтый | Триггер события (не-nil = сработало) |
| `any` | any | Серый | Любой тип |
| `table` | table | Фиолетовый | Таблица/массив |

---

## Основные концепции

### Управление состоянием

`self.state` сохраняется между выполнениями:

```lua
execute = function(self, inputs)
    local state = self.state or { count = 0 }
    state.count = state.count + 1
    self.state = state
    return { count = state.count }
end
```

### Поток событий

События управляют потоком выполнения, основная логика выполняется только при срабатывании:

```lua
execute = function(self, inputs)
    if not inputs.trigger then
        return { result = 0, done = nil }  -- nil = не запускать downstream
    end
    -- Выполнить при срабатывании
    return { result = 42, done = true }
end
```

### Динамические выходные порты

Возврат полей, не определённых в `outputs`, автоматически создаёт динамические порты:

```lua
execute = function(self, inputs)
    local result = { count = 3 }
    -- Динамически генерируем порты dev1_name, dev2_name, dev3_name
    for i = 1, 3 do
        result["dev" .. i .. "_name"] = "Устройство " .. i
    end
    return result
end
```

### Отладка

```lua
execute = function(self, inputs)
    print("Вход:", inputs.value)
    print("Свойство:", self.properties.damage)
    print("Состояние:", self.state)
    return { result = 42 }
end
```

Просмотр в консоли (`Ctrl+`` ). Также можно подключить блок `debug/logger`.

---

## Интерактивные виджеты

Включаются через `meta.widget`:

| widget | Описание | Поле state |
|--------|----------|------------|
| `textinput` | Текстовое поле | `widget_text` |
| `password` | Поле пароля | `widget_text` |
| `textarea` | Многострочный текст | `widget_text` |
| `button` | Кнопка | `widget_checked` |
| `checkbox` | Флажок | `widget_checked` |
| `slider` | Ползунок | `widget_value` |

**Пример: Текстовый ввод**
```lua
return {
    meta = {
        id = "input.text",
        name = "Текстовый ввод",
        widget = "textinput",
        placeholder = "Введите..."
    },
    outputs = {
        { id = "value", name = "Текст", type = "string" }
    },
    execute = function(self, inputs)
        return { value = self.state.widget_text or "" }
    end
}
```

**Пример: Кнопка**
```lua
execute = function(self, inputs)
    local state = self.state or {}
    local was = state.last_checked or false
    local now = self.state.widget_checked or false
    local clicked = now and not was
    state.last_checked = now
    self.state = state
    return { clicked = clicked and true or nil }
end
```

### Свойство hideable

Когда `meta.hideable = true`, в режиме предпросмотра:
- Есть соединения → Скрыт
- Нет соединений → Мини-режим
- При наведении → Временно развернуть

Подходит для: узлов констант, вложений снаряжения, узлов навыков и т.д.

---

## Система анимации

Используйте `self.state._animation` для анимаций смещения позиции:

```lua
self.state._animation = { x = 30, y = 0, speed = 300 }
```

| Параметр | Описание |
|----------|----------|
| `x` | Горизонтальное смещение (положительное=вправо) |
| `y` | Вертикальное смещение (положительное=вниз) |
| `speed` | Скорость (пикс/сек), 0=мгновенно |

**Пример: Выпад атаки**
```lua
if inputs.attack then
    self.state._animation = { x = 30, y = 0, speed = 300 }
else
    self.state._animation = { x = 0, y = 0, speed = 200 }
end
```

---

## Разработка USB

Глобальная таблица `usb` предоставляет полный API для USB-коммуникации.

### Перечисление устройств

```lua
local devices = usb.devices()
for i, dev in ipairs(devices) do
    print(string.format("VID:%04X PID:%04X - %s",
        dev.vendor_id, dev.product_id, dev.product or "Неизвестно"))
end
```

**Поля информации об устройстве:**
| Поле | Тип | Описание |
|------|-----|----------|
| `vendor_id` | number | VID |
| `product_id` | number | PID |
| `bus_number` | number | Номер шины |
| `address` | number | Адрес |
| `speed` | string | "low"/"full"/"high"/"super" |
| `manufacturer` | string? | Производитель |
| `product` | string? | Название продукта |
| `serial_number` | string? | Серийный номер |

### Открытие устройств

```lua
-- По VID/PID
local device = usb.open(0x1234, 0x5678)

-- По адресу шины
local device = usb.open_by_address(1, 5)
```

### Передача данных

**Bulk-передача** (большие данные):
```lua
device:claim_interface(0)
local n = device:write_bulk(0x01, "Hello", 1000)  -- endpoint, данные, timeout_ms
local result = device:read_bulk(0x81, 64, 1000)   -- endpoint, размер, timeout_ms
-- result.data, result.length
```

**Interrupt-передача** (малые данные/низкая задержка):
```lua
device:write_interrupt(0x02, "\x01\x02", 100)
local result = device:read_interrupt(0x82, 8, 100)
```

**Control-передача**:
```lua
local result = device:read_control({
    request_type = usb.request_type("in", "vendor", "device"),
    request = 0x01, value = 0, index = 0, size = 64, timeout = 1000
})
```

### Управление интерфейсами

```lua
device:set_auto_detach_kernel_driver(true)  -- Рекомендуется
device:claim_interface(0)
-- ... операции передачи ...
device:release_interface(0)
```

### Пример USB-блока
```lua
return {
    meta = { id = "usb.scanner", name = "USB Сканер", category = "USB", color = "#9C27B0" },
    outputs = {
        { id = "devices", name = "Список устройств", type = "table" },
        { id = "count", name = "Количество", type = "number" }
    },
    execute = function(self, inputs)
        local devices = usb.devices()
        return { devices = devices, count = #devices }
    end
}
```

### Обработка ошибок

Оборачивайте USB-операции в `pcall`:
```lua
local ok, result = pcall(function()
    local device = usb.open(0x1234, 0x5678)
    device:claim_interface(0)
    return device:read_bulk(0x81, 64, 1000)
end)

if ok then print("OK: " .. result.length)
else print("Ошибка: " .. tostring(result)) end
```

**Частые ошибки:**
| Ошибка | Решение |
|--------|---------|
| Устройство не найдено | Проверьте VID/PID и подключение |
| Доступ запрещён | Linux: udev rules; Windows: Zadig |
| Ресурс занят | Отключите kernel driver |
| Таймаут | Увеличьте timeout |

### Особенности платформ

**Linux** — Создайте `/etc/udev/rules.d/99-usb.rules`:
```
SUBSYSTEM=="usb", ATTR{idVendor}=="1234", MODE="0666"
```

**Windows** — Используйте [Zadig](https://zadig.akeo.ie/) для установки драйвера WinUSB

**macOS** — Используйте `set_auto_detach_kernel_driver(true)`

---

## Лучшие практики

### Соглашения об именовании

| Тип | Соглашение | Пример |
|-----|------------|--------|
| meta.id | `category.name` | `game.attack`, `util.counter` |
| port id | lowercase_underscore | `attack_power`, `is_valid` |
| property id | lowercase_underscore | `max_hp`, `crit_rate` |

### Стиль кода

```lua
-- ✅ Хорошо: ранний возврат, меньше вложенности
execute = function(self, inputs)
    if not inputs.trigger then return { result = 0 } end
    return { result = inputs.value * 2 }
end

-- ❌ Плохо: избыточная вложенность
execute = function(self, inputs)
    if inputs.trigger then
        if inputs.value then
            return { result = inputs.value * 2 }
        end
    end
    return { result = 0 }
end
```

### Советы по производительности

1. **Кэшируйте вычисленные результаты** — Храните неизменяемые данные в `self.state`
2. **Избегайте создания больших таблиц в execute** — Переиспользуйте существующие таблицы
3. **Переиспользуйте USB-устройства** — Кэшируйте открытые устройства в state
4. **Удаляйте print-вызовы** — Удаляйте отладочный вывод в продакшене

### Кодировка файлов

Поддерживаются UTF-8 и GBK, автоопределение. Рекомендуется UTF-8.

---

## Приложение: Структура каталогов

```
scripts/
├── game/        # Игровые сущности
├── lite/        # Lite RPG
├── logic/       # Логическое управление
├── math/        # Математические операции
├── input/       # Интерактивный ввод
├── usb/         # USB-устройства
├── event/       # События
├── util/        # Утилиты
└── debug/       # Отладка
```

## Приложение: Форматы файлов

| Расширение | Формат | Применение |
|------------|--------|------------|
| `.L` | Открытый JSON | Разработка |
| `.LZ` | AES-шифрование | Защита исходников |
| `.lpack` | Зашифрованный пакет | Автономное распространение |

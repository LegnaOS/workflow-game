# Руководство по разработке блоков

## Обзор

Блок — это базовая единица выполнения движка workflow. Каждый блок — это Lua-скрипт, определяющий:

- **Метаданные** — ID, имя, цвет
- **Порты** — входы/выходы
- **Свойства** — настраиваемые параметры
- **Логика выполнения** — Lua-функция

Поместите скрипты в папку `scripts/`, движок автоматически сканирует и загружает с горячей перезагрузкой.

## Структура каталогов

```
scripts/
├── game/           # Игровые сущности
│   ├── character.lua
│   ├── monster.lua
│   └── attack.lua
├── logic/          # Логика
│   ├── branch.lua
│   └── compare.lua
├── math/           # Математика
│   ├── add.lua
│   └── calc.lua
└── util/           # Утилиты
    ├── splitter.lua
    └── merger.lua
```

## Формат скрипта блока

```lua
return {
    -- Метаданные (обязательно)
    meta = {
        id = "category.name",      -- Уникальный ID (обязательно)
        name = "Отображаемое имя", -- Имя в UI
        category = "Категория",    -- Название категории
        description = "Подсказка", -- Описание при наведении
        color = "#4CAF50"          -- Цвет в HEX
    },

    -- Свойства (редактируемые параметры)
    properties = {
        {
            id = "prop_id",
            name = "Свойство",
            type = "number",       -- number/string/boolean
            default = 10,
            min = 0,
            max = 100
        }
    },

    -- Входные порты
    inputs = {
        {
            id = "input_id",
            name = "Вход",
            type = "number",       -- number/string/boolean/event/any
            default = 0
        }
    },

    -- Выходные порты
    outputs = {
        {
            id = "output_id",
            name = "Выход",
            type = "number",
            default = 0
        }
    },

    -- Функция выполнения (основная логика)
    execute = function(self, inputs)
        -- self.properties: доступ к свойствам
        -- self.state: постоянное состояние
        -- inputs: значения входных портов
        
        local result = inputs.input_id * 2
        
        return {
            output_id = result
        }
    end
}
```

## Типы данных

| Тип | Тип Lua | Описание |
|-----|---------|----------|
| `number` | number | Число |
| `string` | string | Строка |
| `boolean` | boolean | Логическое |
| `event` | any/nil | Событие (не-nil = сработало) |
| `any` | any | Любой тип |

## Управление состоянием

Блоки могут сохранять состояние между выполнениями через `self.state`:

```lua
execute = function(self, inputs)
    local state = self.state or {}
    state.count = (state.count or 0) + 1
    self.state = state
    return { count_out = state.count }
end
```

## События

События управляют потоком выполнения:

```lua
inputs = {
    { id = "trigger", name = "Триггер", type = "event" }
},

execute = function(self, inputs)
    if inputs.trigger then
        return { result = 42, event_out = true }
    end
    return { result = 0, event_out = nil }
end
```

## Пример: Блок-счётчик

```lua
return {
    meta = {
        id = "util.counter",
        name = "Счётчик",
        category = "Утилиты",
        color = "#2196F3"
    },

    properties = {
        { id = "step", name = "Шаг", type = "number", default = 1 },
        { id = "max", name = "Максимум", type = "number", default = 100 }
    },

    inputs = {
        { id = "increment", name = "Увеличить", type = "event" },
        { id = "reset", name = "Сброс", type = "event" }
    },

    outputs = {
        { id = "value", name = "Значение", type = "number", default = 0 },
        { id = "overflow", name = "Переполнение", type = "event" }
    },

    execute = function(self, inputs)
        local state = self.state or { value = 0 }
        local props = self.properties
        
        if inputs.reset then
            state.value = 0
        elseif inputs.increment then
            state.value = state.value + (props.step or 1)
        end
        
        local overflow = nil
        if state.value >= (props.max or 100) then
            overflow = true
            state.value = 0
        end
        
        self.state = state
        return { value = state.value, overflow = overflow }
    end
}
```

## Горячая перезагрузка

Сохраните скрипт — движок автоматически перезагрузит. Ошибки выводятся в консоль.

## Форматы файлов

| Расширение | Описание | Применение |
|------------|----------|------------|
| `.L` | Открытый JSON | Разработка |
| `.LZ` | AES-шифрование | Защита от изменений |
| `.dist.L` | Только чтение | Дистрибуция |
| `.dist.LZ` | Шифрованный, только чтение | Релиз |


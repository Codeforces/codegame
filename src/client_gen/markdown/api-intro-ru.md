## Описание API

В пакете для вашего языка программирования вы можете найти файл `MyStrategy.<ext>`/`my_strategy.<ext>`.
Этот файл содержит класс `MyStrategy` с методом `get_action`, где должна быть реализована логика вашей стратегии.

Этот метод будет вызываться каждый тик.

Метод принимает следующие аргументы:

- Доступная информация о текущем состоянии игры,
- Отладочный интерфейс — этот объект позволяет отправлять отладочные команды и запрашивать отладочное состояние приложения прямо из кода вашей стратегии. Заметьте, что этот объект недоступен при тестировании на сервере, а также использовании приложения в консольном режиме (batch mode). Он предназначен только для локальной отладки.

Метод должен вернуть действие, которое вы хотите выполнить в данный тик.

Для отладки существует еще один метод — `debug_update`, принимающий такие же параметры. Он вызывается постоянно во время работы приложения (но не в консольном режиме), если клиент находится в ожидании следующего тика. Метод будет вызван хотя бы раз между тиками.

## Описание объектов

В этой секции, некоторые поля могут быть опциональными (обозначается как `Option<type>`).
Способ реализации зависит от языка.
При возможности используется специальный опциональный (nullable) тип,
иначе другие методы могут быть использованы (например nullable указатели).

Некоторые объекты могут принимать несколько различных форм. Способ реализации зависит от языка.
Если возможно, используется специальный (алгебраический) тип данных,
иначе другие методы могут быть использованы (например варианты представлены классами, унаследованными от абстрактного базового класса).

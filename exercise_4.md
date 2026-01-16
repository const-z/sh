# Паттерны в умном доме

## Цель

Делаем код умного дома более удобным с использованием различных паттернов.

## Описание/Пошаговая инструкция выполнения домашнего задания:

### Билдер

Реализовать билдер для умного дома, позволяющий инициализировать объект умного дома в стиле https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=5d0527e4684f726d54dc375829d983f4.

```rust
fn main() {
    let home = HomeBuilder::new()
        .add_room("First room")
        .add_device("Socket_1", SmartSocket::default())
        .add_device("Socket_2", SmartSocket::default())
        .add_device("Thermo_1", SmartThermo::default())
        .add_room("Second room")
        .add_device("Socket_3", SmartSocket::default())
        .add_device("Thermo_2", SmartThermo::default())
        .build();
}
```

До добавления первой комнаты, билдер запрещает добавлять устройства. Это должно контролироваться компилятором.

### Компоновщик

Реализовать компоновщик для построения отчёта об объектах умного дома в стиле: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c07dfc726e8ccbccdcc2d88a79d3f190.

```rust
fn main() {
    let room = Room::default();
    let device = Device::default();
    let socket1 = Socket::default();
    let socket2 = Socket::default();
    let thermo1 = Thermo::default();
    let thermo2 = Thermo::default();

    let reporter = Reporter::new()
        .add(&room)
        .add(&device)
        .add(&socket1)
        .add(&socket2)
        .add(&thermo1)
        .add(&thermo2)
        .report();
}
```

Использовать статический полиморфизм (дженерики).

Вызов метода report() должен выводить в терминал отчёт обо всех добавленных объектах.

### Observer

Добавить возможность добавления callback-ов в объект комнаты, которые срабатывают при добавлении новых устройств в комнату (паттерн Observer).

Использовать динамический полиморфизм (трейт-объекты).
Можно передавать как объект-subscriber, так и замыкание: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=06e9dc9bcce297d1e80a22d7e9338ee8.

```rust
fn main() {
    let mut room = Room::default();
    room.subscribe(MySubscriber::default());
    room.subscribe(|device| execute_for_added_device(...));
}

trait Subscriber {
    fn on_event(&mut self);
}

struct MySubscriber {
    // ...
}

impl Subscriber for MySubscriber {
    // ...
}
```

Добавить example-ы, демонстрирующие новый функционал.

## Критерии оценки

- Package успешно собирается.
- Приложение-пример успешно выполняется.
- Команды cargo clippy и cargo fmt --check не выводят ошибок и предупреждений.
- Присутствуют и успешно выполняются модульные тесты.

pub struct BankAccount {
    balance: f64, // Приватное поле (по умолчанию)
}

impl BankAccount {
    // Конструктор (статический метод)
    pub fn new(initial_balance: f64) -> Self {
        Self {balance: initial_balance}
    }

    // Публичный метод для внесения денег
    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount
        }
    }

    // Геттер для получения баланса (только чтение)
    pub fn balance(&self) -> f64 {
        self.balance
    }
}

// Общий интерфейс для всех фигур
pub trait Draw {
    fn draw(&self);
}

// Реализация для Кнопки
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Рисуем кнопку: '{}' (размер {}x{})", self.label, self.width, self.height);
    }
}

// Реализация для Текстового поля
pub struct TextField {
    pub placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Рисуем текстовое поле с подсказкой: '{}'", self.placeholder);
    }
}

struct Screen {
    // Список объектов, которые умеют рисовать себя
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 100,
                height: 50,
                label: String::from("Ок"),
            }),
            Box::new(TextField {
                placeholder: String::from("Введите имя..."),
            }),
        ],
    };

    // Вызывает метод draw() у каждого элемента, независимо от его типа
    screen.run();
}

